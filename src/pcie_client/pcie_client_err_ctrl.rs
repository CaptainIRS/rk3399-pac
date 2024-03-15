#[doc = "Register `PCIE_CLIENT_ERR_CTRL` reader"]
pub type R = crate::R<PcieClientErrCtrlSpec>;
#[doc = "Register `PCIE_CLIENT_ERR_CTRL` writer"]
pub type W = crate::W<PcieClientErrCtrlSpec>;
#[doc = "Assert an uncorrectable error input to core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UncorrErrInEn {
    #[doc = "0: write one to generate one pulse The client may activate this input for one cycle to indicate an uncorrectable error detected within the client logic that needs to be reported as an internal error through the PCI Express Advanced Error Reporting mechanism. In response, the core sets the Uncorrectable Internal Error Status bit in the AER Uncorrectable Error Status Register of all enabled Functions, and in EP mode also sends an error message if enabled to do so. This error is not considered Function-specific."]
    B0 = 0,
    #[doc = "1: write one to generate one pulse The client may activate this input for one cycle to indicate an uncorrectable error detected within the client logic that needs to be reported as an internal error through the PCI Express Advanced Error Reporting mechanism. In response, the core sets the Uncorrectable Internal Error Status bit in the AER Uncorrectable Error Status Register of all enabled Functions, and in EP mode also sends an error message if enabled to do so. This error is not considered Function-specific."]
    B1 = 1,
}
impl From<UncorrErrInEn> for bool {
    #[inline(always)]
    fn from(variant: UncorrErrInEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNCORR_ERR_IN_EN` writer - Assert an uncorrectable error input to core"]
pub type UncorrErrInEnW<'a, REG> = crate::BitWriter<'a, REG, UncorrErrInEn>;
impl<'a, REG> UncorrErrInEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write one to generate one pulse The client may activate this input for one cycle to indicate an uncorrectable error detected within the client logic that needs to be reported as an internal error through the PCI Express Advanced Error Reporting mechanism. In response, the core sets the Uncorrectable Internal Error Status bit in the AER Uncorrectable Error Status Register of all enabled Functions, and in EP mode also sends an error message if enabled to do so. This error is not considered Function-specific."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UncorrErrInEn::B0)
    }
    #[doc = "write one to generate one pulse The client may activate this input for one cycle to indicate an uncorrectable error detected within the client logic that needs to be reported as an internal error through the PCI Express Advanced Error Reporting mechanism. In response, the core sets the Uncorrectable Internal Error Status bit in the AER Uncorrectable Error Status Register of all enabled Functions, and in EP mode also sends an error message if enabled to do so. This error is not considered Function-specific."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UncorrErrInEn::B1)
    }
}
#[doc = "Assert a correctable error input to core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CorrErrInEn {
    #[doc = "0: write one to generate one pulse The client may activate this input for one cycle to indicate a correctable error detected within the client logic that needs to be reported as an internal error through the PCI Express Advanced Error Reporting mechanism. In response, the core sets the Corrected Internal Error Status bit in the AER Correctable Error Status Register of all enabled Functions, and in EP mode also sends an error message if enabled to do so. This error is not considered Function-specific."]
    B0 = 0,
    #[doc = "1: write one to generate one pulse The client may activate this input for one cycle to indicate a correctable error detected within the client logic that needs to be reported as an internal error through the PCI Express Advanced Error Reporting mechanism. In response, the core sets the Corrected Internal Error Status bit in the AER Correctable Error Status Register of all enabled Functions, and in EP mode also sends an error message if enabled to do so. This error is not considered Function-specific."]
    B1 = 1,
}
impl From<CorrErrInEn> for bool {
    #[inline(always)]
    fn from(variant: CorrErrInEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORR_ERR_IN_EN` writer - Assert a correctable error input to core"]
pub type CorrErrInEnW<'a, REG> = crate::BitWriter<'a, REG, CorrErrInEn>;
impl<'a, REG> CorrErrInEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write one to generate one pulse The client may activate this input for one cycle to indicate a correctable error detected within the client logic that needs to be reported as an internal error through the PCI Express Advanced Error Reporting mechanism. In response, the core sets the Corrected Internal Error Status bit in the AER Correctable Error Status Register of all enabled Functions, and in EP mode also sends an error message if enabled to do so. This error is not considered Function-specific."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CorrErrInEn::B0)
    }
    #[doc = "write one to generate one pulse The client may activate this input for one cycle to indicate a correctable error detected within the client logic that needs to be reported as an internal error through the PCI Express Advanced Error Reporting mechanism. In response, the core sets the Corrected Internal Error Status bit in the AER Correctable Error Status Register of all enabled Functions, and in EP mode also sends an error message if enabled to do so. This error is not considered Function-specific."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CorrErrInEn::B1)
    }
}
#[doc = "Enable fatal error counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FatalErrCntEn {
    #[doc = "0: enable counter"]
    B0 = 0,
    #[doc = "1: enable counter"]
    B1 = 1,
}
impl From<FatalErrCntEn> for bool {
    #[inline(always)]
    fn from(variant: FatalErrCntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FATAL_ERR_CNT_EN` reader - Enable fatal error counter"]
pub type FatalErrCntEnR = crate::BitReader<FatalErrCntEn>;
impl FatalErrCntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FatalErrCntEn {
        match self.bits {
            false => FatalErrCntEn::B0,
            true => FatalErrCntEn::B1,
        }
    }
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FatalErrCntEn::B0
    }
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FatalErrCntEn::B1
    }
}
#[doc = "Field `FATAL_ERR_CNT_EN` writer - Enable fatal error counter"]
pub type FatalErrCntEnW<'a, REG> = crate::BitWriter<'a, REG, FatalErrCntEn>;
impl<'a, REG> FatalErrCntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FatalErrCntEn::B0)
    }
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FatalErrCntEn::B1)
    }
}
#[doc = "Enable non-fatal error counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NfatalErrCntEn {
    #[doc = "0: enable counter"]
    B0 = 0,
    #[doc = "1: enable counter"]
    B1 = 1,
}
impl From<NfatalErrCntEn> for bool {
    #[inline(always)]
    fn from(variant: NfatalErrCntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFATAL_ERR_CNT_EN` reader - Enable non-fatal error counter"]
pub type NfatalErrCntEnR = crate::BitReader<NfatalErrCntEn>;
impl NfatalErrCntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NfatalErrCntEn {
        match self.bits {
            false => NfatalErrCntEn::B0,
            true => NfatalErrCntEn::B1,
        }
    }
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NfatalErrCntEn::B0
    }
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NfatalErrCntEn::B1
    }
}
#[doc = "Field `NFATAL_ERR_CNT_EN` writer - Enable non-fatal error counter"]
pub type NfatalErrCntEnW<'a, REG> = crate::BitWriter<'a, REG, NfatalErrCntEn>;
impl<'a, REG> NfatalErrCntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(NfatalErrCntEn::B0)
    }
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(NfatalErrCntEn::B1)
    }
}
#[doc = "Enable correctable error counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CorrErrCntEn {
    #[doc = "0: enable counter"]
    B0 = 0,
    #[doc = "1: enable counter"]
    B1 = 1,
}
impl From<CorrErrCntEn> for bool {
    #[inline(always)]
    fn from(variant: CorrErrCntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORR_ERR_CNT_EN` reader - Enable correctable error counter"]
pub type CorrErrCntEnR = crate::BitReader<CorrErrCntEn>;
impl CorrErrCntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CorrErrCntEn {
        match self.bits {
            false => CorrErrCntEn::B0,
            true => CorrErrCntEn::B1,
        }
    }
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CorrErrCntEn::B0
    }
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CorrErrCntEn::B1
    }
}
#[doc = "Field `CORR_ERR_CNT_EN` writer - Enable correctable error counter"]
pub type CorrErrCntEnW<'a, REG> = crate::BitWriter<'a, REG, CorrErrCntEn>;
impl<'a, REG> CorrErrCntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CorrErrCntEn::B0)
    }
    #[doc = "enable counter"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CorrErrCntEn::B1)
    }
}
#[doc = "Write mask For each served bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum WriteMask {
    #[doc = "0: write enable"]
    B0 = 0,
    #[doc = "1: write enable"]
    B1 = 1,
}
impl From<WriteMask> for u16 {
    #[inline(always)]
    fn from(variant: WriteMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteMask {
    type Ux = u16;
}
#[doc = "Field `WRITE_MASK` writer - Write mask For each served bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, WriteMask>;
impl<'a, REG> WriteMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "write enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B0)
    }
    #[doc = "write enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B1)
    }
}
impl R {
    #[doc = "Bit 8 - Enable fatal error counter"]
    #[inline(always)]
    pub fn fatal_err_cnt_en(&self) -> FatalErrCntEnR {
        FatalErrCntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable non-fatal error counter"]
    #[inline(always)]
    pub fn nfatal_err_cnt_en(&self) -> NfatalErrCntEnR {
        NfatalErrCntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable correctable error counter"]
    #[inline(always)]
    pub fn corr_err_cnt_en(&self) -> CorrErrCntEnR {
        CorrErrCntEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assert an uncorrectable error input to core"]
    #[inline(always)]
    #[must_use]
    pub fn uncorr_err_in_en(&mut self) -> UncorrErrInEnW<PcieClientErrCtrlSpec> {
        UncorrErrInEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Assert a correctable error input to core"]
    #[inline(always)]
    #[must_use]
    pub fn corr_err_in_en(&mut self) -> CorrErrInEnW<PcieClientErrCtrlSpec> {
        CorrErrInEnW::new(self, 1)
    }
    #[doc = "Bit 8 - Enable fatal error counter"]
    #[inline(always)]
    #[must_use]
    pub fn fatal_err_cnt_en(&mut self) -> FatalErrCntEnW<PcieClientErrCtrlSpec> {
        FatalErrCntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable non-fatal error counter"]
    #[inline(always)]
    #[must_use]
    pub fn nfatal_err_cnt_en(&mut self) -> NfatalErrCntEnW<PcieClientErrCtrlSpec> {
        NfatalErrCntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable correctable error counter"]
    #[inline(always)]
    #[must_use]
    pub fn corr_err_cnt_en(&mut self) -> CorrErrCntEnW<PcieClientErrCtrlSpec> {
        CorrErrCntEnW::new(self, 10)
    }
    #[doc = "Bits 16:31 - Write mask For each served bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PcieClientErrCtrlSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Error control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_err_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_err_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientErrCtrlSpec;
impl crate::RegisterSpec for PcieClientErrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_err_ctrl::R`](R) reader structure"]
impl crate::Readable for PcieClientErrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_err_ctrl::W`](W) writer structure"]
impl crate::Writable for PcieClientErrCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_CLIENT_ERR_CTRL to value 0"]
impl crate::Resettable for PcieClientErrCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
