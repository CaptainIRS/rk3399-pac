#[doc = "Register `I2S_RXCR` reader"]
pub type R = crate::R<I2sRxcrSpec>;
#[doc = "Register `I2S_RXCR` writer"]
pub type W = crate::W<I2sRxcrSpec>;
#[doc = "Field `VDW` reader - Valid Data width (Can be written only when XFER\\[1\\]
bit is 0.) 0~14:reserved 15:16bit 16:17bit 17:18bit 18:19bit ...... n:(n+1)bit ...... 28:29bit 29:30bit 30:31bit 31:32bit"]
pub type VdwR = crate::FieldReader;
#[doc = "Field `VDW` writer - Valid Data width (Can be written only when XFER\\[1\\]
bit is 0.) 0~14:reserved 15:16bit 16:17bit 17:18bit 18:19bit ...... n:(n+1)bit ...... 28:29bit 29:30bit 30:31bit 31:32bit"]
pub type VdwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Transfer format select (Can be written only when XFER\\[1\\]
bit is 0.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfs {
    #[doc = "0: pcm"]
    B0 = 0,
    #[doc = "1: pcm"]
    B1 = 1,
}
impl From<Tfs> for bool {
    #[inline(always)]
    fn from(variant: Tfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFS` reader - Transfer format select (Can be written only when XFER\\[1\\]
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
    #[doc = "pcm"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tfs::B0
    }
    #[doc = "pcm"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tfs::B1
    }
}
#[doc = "Field `TFS` writer - Transfer format select (Can be written only when XFER\\[1\\]
bit is 0.)"]
pub type TfsW<'a, REG> = crate::BitWriter<'a, REG, Tfs>;
impl<'a, REG> TfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pcm"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfs::B0)
    }
    #[doc = "pcm"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfs::B1)
    }
}
#[doc = "PCM bus mode (Can be written only when XFER\\[1\\]
bit is 0.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pbm {
    #[doc = "0: PCM delay 3 mode"]
    D0 = 0,
    #[doc = "1: PCM delay 3 mode"]
    D1 = 1,
    #[doc = "2: PCM delay 3 mode"]
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
#[doc = "Field `PBM` reader - PCM bus mode (Can be written only when XFER\\[1\\]
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
    #[doc = "PCM delay 3 mode"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Pbm::D0
    }
    #[doc = "PCM delay 3 mode"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Pbm::D1
    }
    #[doc = "PCM delay 3 mode"]
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
#[doc = "Field `PBM` writer - PCM bus mode (Can be written only when XFER\\[1\\]
bit is 0.)"]
pub type PbmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Pbm>;
impl<'a, REG> PbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCM delay 3 mode"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Pbm::D0)
    }
    #[doc = "PCM delay 3 mode"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Pbm::D1)
    }
    #[doc = "PCM delay 3 mode"]
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
#[doc = "I2S bus mode (Can be written only when XFER\\[1\\]
bit is 0.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ibm {
    #[doc = "0: reserved"]
    D0 = 0,
    #[doc = "1: reserved"]
    D1 = 1,
    #[doc = "2: reserved"]
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
#[doc = "Field `IBM` reader - I2S bus mode (Can be written only when XFER\\[1\\]
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
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Ibm::D0
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Ibm::D1
    }
    #[doc = "reserved"]
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
#[doc = "Field `IBM` writer - I2S bus mode (Can be written only when XFER\\[1\\]
bit is 0.)"]
pub type IbmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ibm>;
impl<'a, REG> IbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Ibm::D0)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Ibm::D1)
    }
    #[doc = "reserved"]
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
#[doc = "First Bit Mode (Can be written only when XFER\\[1\\]
bit is 0.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fbm {
    #[doc = "0: LSB"]
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
#[doc = "Field `FBM` reader - First Bit Mode (Can be written only when XFER\\[1\\]
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
    #[doc = "LSB"]
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
#[doc = "Field `FBM` writer - First Bit Mode (Can be written only when XFER\\[1\\]
bit is 0.)"]
pub type FbmW<'a, REG> = crate::BitWriter<'a, REG, Fbm>;
impl<'a, REG> FbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB"]
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
#[doc = "Store justified mode (Can be written only when XFER\\[1\\]
bit is 0.) 16bit~31bit DATA stored in 32 bits width fifo. If VDW select 16bit data, this bit is valid only when HWT select 0.Because if HWT is 1, every fifo unit contain two 16bit data and 32 bit space is full, it is impossible to choose justified mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sjm {
    #[doc = "0: left justified"]
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
#[doc = "Field `SJM` reader - Store justified mode (Can be written only when XFER\\[1\\]
bit is 0.) 16bit~31bit DATA stored in 32 bits width fifo. If VDW select 16bit data, this bit is valid only when HWT select 0.Because if HWT is 1, every fifo unit contain two 16bit data and 32 bit space is full, it is impossible to choose justified mode."]
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
    #[doc = "left justified"]
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
#[doc = "Field `SJM` writer - Store justified mode (Can be written only when XFER\\[1\\]
bit is 0.) 16bit~31bit DATA stored in 32 bits width fifo. If VDW select 16bit data, this bit is valid only when HWT select 0.Because if HWT is 1, every fifo unit contain two 16bit data and 32 bit space is full, it is impossible to choose justified mode."]
pub type SjmW<'a, REG> = crate::BitWriter<'a, REG, Sjm>;
impl<'a, REG> SjmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "left justified"]
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
#[doc = "Halfword word transform (Can be written only when XFER\\[1\\]
bit is 0.) Only valid when VDW select 16bit data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwt {
    #[doc = "0: low 16bit data valid to AHB/APB bus, high 16 bit data invalid."]
    B0 = 0,
    #[doc = "1: low 16bit data valid to AHB/APB bus, high 16 bit data invalid."]
    B1 = 1,
}
impl From<Hwt> for bool {
    #[inline(always)]
    fn from(variant: Hwt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWT` reader - Halfword word transform (Can be written only when XFER\\[1\\]
bit is 0.) Only valid when VDW select 16bit data."]
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
    #[doc = "low 16bit data valid to AHB/APB bus, high 16 bit data invalid."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hwt::B0
    }
    #[doc = "low 16bit data valid to AHB/APB bus, high 16 bit data invalid."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hwt::B1
    }
}
#[doc = "Field `HWT` writer - Halfword word transform (Can be written only when XFER\\[1\\]
bit is 0.) Only valid when VDW select 16bit data."]
pub type HwtW<'a, REG> = crate::BitWriter<'a, REG, Hwt>;
impl<'a, REG> HwtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "low 16bit data valid to AHB/APB bus, high 16 bit data invalid."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hwt::B0)
    }
    #[doc = "low 16bit data valid to AHB/APB bus, high 16 bit data invalid."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hwt::B1)
    }
}
#[doc = "RX Channel select register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcsr {
    #[doc = "0: eight channel"]
    B00 = 0,
    #[doc = "1: eight channel"]
    B01 = 1,
    #[doc = "2: eight channel"]
    B10 = 2,
    #[doc = "3: eight channel"]
    B11 = 3,
}
impl From<Rcsr> for u8 {
    #[inline(always)]
    fn from(variant: Rcsr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcsr {
    type Ux = u8;
}
#[doc = "Field `RCSR` reader - RX Channel select register"]
pub type RcsrR = crate::FieldReader<Rcsr>;
impl RcsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcsr {
        match self.bits {
            0 => Rcsr::B00,
            1 => Rcsr::B01,
            2 => Rcsr::B10,
            3 => Rcsr::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rcsr::B00
    }
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rcsr::B01
    }
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Rcsr::B10
    }
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Rcsr::B11
    }
}
#[doc = "Field `RCSR` writer - RX Channel select register"]
pub type RcsrW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rcsr>;
impl<'a, REG> RcsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rcsr::B00)
    }
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rcsr::B01)
    }
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Rcsr::B10)
    }
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Rcsr::B11)
    }
}
impl R {
    #[doc = "Bits 0:4 - Valid Data width (Can be written only when XFER\\[1\\]
bit is 0.) 0~14:reserved 15:16bit 16:17bit 17:18bit 18:19bit ...... n:(n+1)bit ...... 28:29bit 29:30bit 30:31bit 31:32bit"]
    #[inline(always)]
    pub fn vdw(&self) -> VdwR {
        VdwR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Transfer format select (Can be written only when XFER\\[1\\]
bit is 0.)"]
    #[inline(always)]
    pub fn tfs(&self) -> TfsR {
        TfsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:8 - PCM bus mode (Can be written only when XFER\\[1\\]
bit is 0.)"]
    #[inline(always)]
    pub fn pbm(&self) -> PbmR {
        PbmR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - I2S bus mode (Can be written only when XFER\\[1\\]
bit is 0.)"]
    #[inline(always)]
    pub fn ibm(&self) -> IbmR {
        IbmR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - First Bit Mode (Can be written only when XFER\\[1\\]
bit is 0.)"]
    #[inline(always)]
    pub fn fbm(&self) -> FbmR {
        FbmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Store justified mode (Can be written only when XFER\\[1\\]
bit is 0.) 16bit~31bit DATA stored in 32 bits width fifo. If VDW select 16bit data, this bit is valid only when HWT select 0.Because if HWT is 1, every fifo unit contain two 16bit data and 32 bit space is full, it is impossible to choose justified mode."]
    #[inline(always)]
    pub fn sjm(&self) -> SjmR {
        SjmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Halfword word transform (Can be written only when XFER\\[1\\]
bit is 0.) Only valid when VDW select 16bit data."]
    #[inline(always)]
    pub fn hwt(&self) -> HwtR {
        HwtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - RX Channel select register"]
    #[inline(always)]
    pub fn rcsr(&self) -> RcsrR {
        RcsrR::new(((self.bits >> 15) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Valid Data width (Can be written only when XFER\\[1\\]
bit is 0.) 0~14:reserved 15:16bit 16:17bit 17:18bit 18:19bit ...... n:(n+1)bit ...... 28:29bit 29:30bit 30:31bit 31:32bit"]
    #[inline(always)]
    #[must_use]
    pub fn vdw(&mut self) -> VdwW<I2sRxcrSpec> {
        VdwW::new(self, 0)
    }
    #[doc = "Bit 5 - Transfer format select (Can be written only when XFER\\[1\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn tfs(&mut self) -> TfsW<I2sRxcrSpec> {
        TfsW::new(self, 5)
    }
    #[doc = "Bits 7:8 - PCM bus mode (Can be written only when XFER\\[1\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn pbm(&mut self) -> PbmW<I2sRxcrSpec> {
        PbmW::new(self, 7)
    }
    #[doc = "Bits 9:10 - I2S bus mode (Can be written only when XFER\\[1\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn ibm(&mut self) -> IbmW<I2sRxcrSpec> {
        IbmW::new(self, 9)
    }
    #[doc = "Bit 11 - First Bit Mode (Can be written only when XFER\\[1\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn fbm(&mut self) -> FbmW<I2sRxcrSpec> {
        FbmW::new(self, 11)
    }
    #[doc = "Bit 12 - Store justified mode (Can be written only when XFER\\[1\\]
bit is 0.) 16bit~31bit DATA stored in 32 bits width fifo. If VDW select 16bit data, this bit is valid only when HWT select 0.Because if HWT is 1, every fifo unit contain two 16bit data and 32 bit space is full, it is impossible to choose justified mode."]
    #[inline(always)]
    #[must_use]
    pub fn sjm(&mut self) -> SjmW<I2sRxcrSpec> {
        SjmW::new(self, 12)
    }
    #[doc = "Bit 14 - Halfword word transform (Can be written only when XFER\\[1\\]
bit is 0.) Only valid when VDW select 16bit data."]
    #[inline(always)]
    #[must_use]
    pub fn hwt(&mut self) -> HwtW<I2sRxcrSpec> {
        HwtW::new(self, 14)
    }
    #[doc = "Bits 15:16 - RX Channel select register"]
    #[inline(always)]
    #[must_use]
    pub fn rcsr(&mut self) -> RcsrW<I2sRxcrSpec> {
        RcsrW::new(self, 15)
    }
}
#[doc = "receive operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_rxcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_rxcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sRxcrSpec;
impl crate::RegisterSpec for I2sRxcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_rxcr::R`](R) reader structure"]
impl crate::Readable for I2sRxcrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_rxcr::W`](W) writer structure"]
impl crate::Writable for I2sRxcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2S_RXCR to value 0x0f"]
impl crate::Resettable for I2sRxcrSpec {
    const RESET_VALUE: u32 = 0x0f;
}
