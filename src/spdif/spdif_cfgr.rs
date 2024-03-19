#[doc = "Register `SPDIF_CFGR` reader"]
pub type R = crate::R<SpdifCfgrSpec>;
#[doc = "Register `SPDIF_CFGR` writer"]
pub type W = crate::W<SpdifCfgrSpec>;
#[doc = "Valid data width\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vdw {
    #[doc = "0: 16bit"]
    B00 = 0,
    #[doc = "1: 20bit"]
    B01 = 1,
    #[doc = "2: 24bit"]
    B10 = 2,
    #[doc = "3: reserved The valid data width is 16bit only for non-linear PCM"]
    B11 = 3,
}
impl From<Vdw> for u8 {
    #[inline(always)]
    fn from(variant: Vdw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vdw {
    type Ux = u8;
}
#[doc = "Field `VDW` reader - Valid data width"]
pub type VdwR = crate::FieldReader<Vdw>;
impl VdwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdw {
        match self.bits {
            0 => Vdw::B00,
            1 => Vdw::B01,
            2 => Vdw::B10,
            3 => Vdw::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "16bit"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Vdw::B00
    }
    #[doc = "20bit"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Vdw::B01
    }
    #[doc = "24bit"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Vdw::B10
    }
    #[doc = "reserved The valid data width is 16bit only for non-linear PCM"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Vdw::B11
    }
}
#[doc = "Field `VDW` writer - Valid data width"]
pub type VdwW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Vdw>;
impl<'a, REG> VdwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16bit"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Vdw::B00)
    }
    #[doc = "20bit"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Vdw::B01)
    }
    #[doc = "24bit"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Vdw::B10)
    }
    #[doc = "reserved The valid data width is 16bit only for non-linear PCM"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Vdw::B11)
    }
}
#[doc = "Halfword word transform enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwt {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable It is valid only when the valid data width is 16bit."]
    B1 = 1,
}
impl From<Hwt> for bool {
    #[inline(always)]
    fn from(variant: Hwt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWT` reader - Halfword word transform enable"]
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
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hwt::B0
    }
    #[doc = "enable It is valid only when the valid data width is 16bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hwt::B1
    }
}
#[doc = "Field `HWT` writer - Halfword word transform enable"]
pub type HwtW<'a, REG> = crate::BitWriter<'a, REG, Hwt>;
impl<'a, REG> HwtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hwt::B0)
    }
    #[doc = "enable It is valid only when the valid data width is 16bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hwt::B1)
    }
}
#[doc = "audio data justified\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adj {
    #[doc = "0: Right justified"]
    B0 = 0,
    #[doc = "1: Left justified"]
    B1 = 1,
}
impl From<Adj> for bool {
    #[inline(always)]
    fn from(variant: Adj) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADJ` reader - audio data justified"]
pub type AdjR = crate::BitReader<Adj>;
impl AdjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adj {
        match self.bits {
            false => Adj::B0,
            true => Adj::B1,
        }
    }
    #[doc = "Right justified"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adj::B0
    }
    #[doc = "Left justified"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adj::B1
    }
}
#[doc = "Field `ADJ` writer - audio data justified"]
pub type AdjW<'a, REG> = crate::BitWriter<'a, REG, Adj>;
impl<'a, REG> AdjW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right justified"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adj::B0)
    }
    #[doc = "Left justified"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adj::B1)
    }
}
#[doc = "Validity flag enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vfe {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Vfe> for bool {
    #[inline(always)]
    fn from(variant: Vfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VFE` reader - Validity flag enable"]
pub type VfeR = crate::BitReader<Vfe>;
impl VfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vfe {
        match self.bits {
            false => Vfe::B0,
            true => Vfe::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vfe::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vfe::B1
    }
}
#[doc = "Field `VFE` writer - Validity flag enable"]
pub type VfeW<'a, REG> = crate::BitWriter<'a, REG, Vfe>;
impl<'a, REG> VfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Vfe::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Vfe::B1)
    }
}
#[doc = "User data enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ude {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Ude> for bool {
    #[inline(always)]
    fn from(variant: Ude) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDE` reader - User data enable"]
pub type UdeR = crate::BitReader<Ude>;
impl UdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ude {
        match self.bits {
            false => Ude::B0,
            true => Ude::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ude::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ude::B1
    }
}
#[doc = "Field `UDE` writer - User data enable"]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG, Ude>;
impl<'a, REG> UdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ude::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ude::B1)
    }
}
#[doc = "Channel status enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cse {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable The bit should be set to 1 when the channel conveys non-linear PCM"]
    B1 = 1,
}
impl From<Cse> for bool {
    #[inline(always)]
    fn from(variant: Cse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSE` reader - Channel status enable"]
pub type CseR = crate::BitReader<Cse>;
impl CseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cse {
        match self.bits {
            false => Cse::B0,
            true => Cse::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cse::B0
    }
    #[doc = "enable The bit should be set to 1 when the channel conveys non-linear PCM"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cse::B1
    }
}
#[doc = "Field `CSE` writer - Channel status enable"]
pub type CseW<'a, REG> = crate::BitWriter<'a, REG, Cse>;
impl<'a, REG> CseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cse::B0)
    }
    #[doc = "enable The bit should be set to 1 when the channel conveys non-linear PCM"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cse::B1)
    }
}
#[doc = "Field `CLR` writer - mclk domain logic clear\n\nWrite 1 to clear mclk domain logic. Read return zero."]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PCM type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcmtype {
    #[doc = "0: linear PCM"]
    B0 = 0,
    #[doc = "1: non-linear PCM"]
    B1 = 1,
}
impl From<Pcmtype> for bool {
    #[inline(always)]
    fn from(variant: Pcmtype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCMTYPE` reader - PCM type"]
pub type PcmtypeR = crate::BitReader<Pcmtype>;
impl PcmtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcmtype {
        match self.bits {
            false => Pcmtype::B0,
            true => Pcmtype::B1,
        }
    }
    #[doc = "linear PCM"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pcmtype::B0
    }
    #[doc = "non-linear PCM"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pcmtype::B1
    }
}
#[doc = "Field `PCMTYPE` writer - PCM type"]
pub type PcmtypeW<'a, REG> = crate::BitWriter<'a, REG, Pcmtype>;
impl<'a, REG> PcmtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "linear PCM"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmtype::B0)
    }
    #[doc = "non-linear PCM"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcmtype::B1)
    }
}
#[doc = "Field `MCD` reader - mclk divider\n\nFmclk/Fsdo\n\nThis parameter can be calculated by Fmclk/(Fs*128).\n\nFs=the sample frequency be wanted"]
pub type McdR = crate::FieldReader;
#[doc = "Field `MCD` writer - mclk divider\n\nFmclk/Fsdo\n\nThis parameter can be calculated by Fmclk/(Fs*128).\n\nFs=the sample frequency be wanted"]
pub type McdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Valid data width"]
    #[inline(always)]
    pub fn vdw(&self) -> VdwR {
        VdwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Halfword word transform enable"]
    #[inline(always)]
    pub fn hwt(&self) -> HwtR {
        HwtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - audio data justified"]
    #[inline(always)]
    pub fn adj(&self) -> AdjR {
        AdjR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Validity flag enable"]
    #[inline(always)]
    pub fn vfe(&self) -> VfeR {
        VfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - User data enable"]
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel status enable"]
    #[inline(always)]
    pub fn cse(&self) -> CseR {
        CseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - PCM type"]
    #[inline(always)]
    pub fn pcmtype(&self) -> PcmtypeR {
        PcmtypeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - mclk divider\n\nFmclk/Fsdo\n\nThis parameter can be calculated by Fmclk/(Fs*128).\n\nFs=the sample frequency be wanted"]
    #[inline(always)]
    pub fn mcd(&self) -> McdR {
        McdR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Valid data width"]
    #[inline(always)]
    #[must_use]
    pub fn vdw(&mut self) -> VdwW<SpdifCfgrSpec> {
        VdwW::new(self, 0)
    }
    #[doc = "Bit 2 - Halfword word transform enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwt(&mut self) -> HwtW<SpdifCfgrSpec> {
        HwtW::new(self, 2)
    }
    #[doc = "Bit 3 - audio data justified"]
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> AdjW<SpdifCfgrSpec> {
        AdjW::new(self, 3)
    }
    #[doc = "Bit 4 - Validity flag enable"]
    #[inline(always)]
    #[must_use]
    pub fn vfe(&mut self) -> VfeW<SpdifCfgrSpec> {
        VfeW::new(self, 4)
    }
    #[doc = "Bit 5 - User data enable"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UdeW<SpdifCfgrSpec> {
        UdeW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel status enable"]
    #[inline(always)]
    #[must_use]
    pub fn cse(&mut self) -> CseW<SpdifCfgrSpec> {
        CseW::new(self, 6)
    }
    #[doc = "Bit 7 - mclk domain logic clear\n\nWrite 1 to clear mclk domain logic. Read return zero."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<SpdifCfgrSpec> {
        ClrW::new(self, 7)
    }
    #[doc = "Bit 8 - PCM type"]
    #[inline(always)]
    #[must_use]
    pub fn pcmtype(&mut self) -> PcmtypeW<SpdifCfgrSpec> {
        PcmtypeW::new(self, 8)
    }
    #[doc = "Bits 16:23 - mclk divider\n\nFmclk/Fsdo\n\nThis parameter can be calculated by Fmclk/(Fs*128).\n\nFs=the sample frequency be wanted"]
    #[inline(always)]
    #[must_use]
    pub fn mcd(&mut self) -> McdW<SpdifCfgrSpec> {
        McdW::new(self, 16)
    }
}
#[doc = "Transfer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifCfgrSpec;
impl crate::RegisterSpec for SpdifCfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_cfgr::R`](R) reader structure"]
impl crate::Readable for SpdifCfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdif_cfgr::W`](W) writer structure"]
impl crate::Writable for SpdifCfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIF_CFGR to value 0"]
impl crate::Resettable for SpdifCfgrSpec {
    const RESET_VALUE: u32 = 0;
}
