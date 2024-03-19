#[doc = "Register `PCIE_PF_CORRECTABLE_ERROR_STATUS` reader"]
pub type R = crate::R<PciePfCorrectableErrorStatusSpec>;
#[doc = "Register `PCIE_PF_CORRECTABLE_ERROR_STATUS` writer"]
pub type W = crate::W<PciePfCorrectableErrorStatusSpec>;
#[doc = "Field `RES` reader - Receiver Error Status \\[RES\\]\n\nThis bit is set when an error is\n\ndetected in the receive side of the\n\nPhysical Layer of the core (e.g. a bit\n\nerror or coding violation). This error\n\nis not Function-specific. STICKY."]
pub type ResR = crate::BitReader;
#[doc = "Field `RES` writer - Receiver Error Status \\[RES\\]\n\nThis bit is set when an error is\n\ndetected in the receive side of the\n\nPhysical Layer of the core (e.g. a bit\n\nerror or coding violation). This error\n\nis not Function-specific. STICKY."]
pub type ResW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R12` reader - Reserved \\[R12\\]\n\nReserved"]
pub type R12R = crate::FieldReader;
#[doc = "Field `BTS` reader - Bad TP Status \\[BTS\\]\n\nThis bit is set when an error is\n\ndetected in a received TLP by the\n\nData Link Layer of the core. The\n\nconditions causing this error are: (i)\n\nAn LCRC error (ii) The packet\n\nterminates with EDB symbol, but its\n\nLCRC field does not equal the\n\ninverted value of the calculated CRC.\n\nThis error is not Function-specific.\n\nSTICKY."]
pub type BtsR = crate::BitReader;
#[doc = "Field `BTS` writer - Bad TP Status \\[BTS\\]\n\nThis bit is set when an error is\n\ndetected in a received TLP by the\n\nData Link Layer of the core. The\n\nconditions causing this error are: (i)\n\nAn LCRC error (ii) The packet\n\nterminates with EDB symbol, but its\n\nLCRC field does not equal the\n\ninverted value of the calculated CRC.\n\nThis error is not Function-specific.\n\nSTICKY."]
pub type BtsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BDS` reader - Bad DLLP Status \\[BDS\\]\n\nThis bit is set when an LCRC error is\n\ndetected in a received DLLP, and no\n\nerrors were detected by the Physical\n\nLayer. This error is not Function-\n\nspecific. STICKY."]
pub type BdsR = crate::BitReader;
#[doc = "Field `BDS` writer - Bad DLLP Status \\[BDS\\]\n\nThis bit is set when an LCRC error is\n\ndetected in a received DLLP, and no\n\nerrors were detected by the Physical\n\nLayer. This error is not Function-\n\nspecific. STICKY."]
pub type BdsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RNRS` reader - Replay Number Rollover Status \\[RNRS\\]\n\nThis bit is set when the replay count\n\nrolls over after three re-\n\ntransmissions of a TLP at the Data\n\nLink Layer of the core. This error is\n\nnot Function- specific STICKY."]
pub type RnrsR = crate::BitReader;
#[doc = "Field `RNRS` writer - Replay Number Rollover Status \\[RNRS\\]\n\nThis bit is set when the replay count\n\nrolls over after three re-\n\ntransmissions of a TLP at the Data\n\nLink Layer of the core. This error is\n\nnot Function- specific STICKY."]
pub type RnrsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R13` reader - Reserved \\[R13\\]\n\nReserved"]
pub type R13R = crate::FieldReader;
#[doc = "Field `RTTS` reader - Replay Timer Timeout Status \\[RTTS\\]\n\nThis bit is set when the replay timer\n\nin the Data Link Layer of the core\n\ntimes out, causing the core to\n\nretransmit a TLP. This error is not\n\nFunction- specific. STICKY."]
pub type RttsR = crate::BitReader;
#[doc = "Field `RTTS` writer - Replay Timer Timeout Status \\[RTTS\\]\n\nThis bit is set when the replay timer\n\nin the Data Link Layer of the core\n\ntimes out, causing the core to\n\nretransmit a TLP. This error is not\n\nFunction- specific. STICKY."]
pub type RttsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ANFES` reader - Advisory Non- Fatal Error Status \\[ANFES\\]\n\nThis bit is set when an uncorrectable\n\nerror occurs, which is determined to\n\nbelong to one of the special cases\n\ndescribed in Section 6.2.3.2.4 of the\n\nPCI Express 2.0 Specifications. This\n\ncauses the core to generate an\n\nERR_COR message in place of an\n\nERR_NONFATAL message. STICKY."]
pub type AnfesR = crate::BitReader;
#[doc = "Field `ANFES` writer - Advisory Non- Fatal Error Status \\[ANFES\\]\n\nThis bit is set when an uncorrectable\n\nerror occurs, which is determined to\n\nbelong to one of the special cases\n\ndescribed in Section 6.2.3.2.4 of the\n\nPCI Express 2.0 Specifications. This\n\ncauses the core to generate an\n\nERR_COR message in place of an\n\nERR_NONFATAL message. STICKY."]
pub type AnfesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIES` reader - Corrected Internal Error Status \\[CIES\\]\n\nThis bit is set when the core has\n\ndetected an internal correctable\n\nerror condition (a correctable ECC\n\nerror while reading from any of the\n\nRAMs). This bit is also set in\n\nresponse to the client signaling an\n\ninternal error through the input\n\nCORRECTABLE_ERROR_IN. This\n\nerror is not Function-specific.\n\nSTICKY."]
pub type CiesR = crate::BitReader;
#[doc = "Field `CIES` writer - Corrected Internal Error Status \\[CIES\\]\n\nThis bit is set when the core has\n\ndetected an internal correctable\n\nerror condition (a correctable ECC\n\nerror while reading from any of the\n\nRAMs). This bit is also set in\n\nresponse to the client signaling an\n\ninternal error through the input\n\nCORRECTABLE_ERROR_IN. This\n\nerror is not Function-specific.\n\nSTICKY."]
pub type CiesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HLOS` reader - Header Log Overflow Status \\[HLOS\\]\n\nThis bit is set on a Header Log\n\nRegister overflow, that is, when the\n\nheader could not be logged in the\n\nHeader Log Register because it is\n\noccupied by a previous header.\n\nSTICKY."]
pub type HlosR = crate::BitReader;
#[doc = "Field `HLOS` writer - Header Log Overflow Status \\[HLOS\\]\n\nThis bit is set on a Header Log\n\nRegister overflow, that is, when the\n\nheader could not be logged in the\n\nHeader Log Register because it is\n\noccupied by a previous header.\n\nSTICKY."]
pub type HlosW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R14` reader - Reserved \\[R14\\]\n\nReserved"]
pub type R14R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receiver Error Status \\[RES\\]\n\nThis bit is set when an error is\n\ndetected in the receive side of the\n\nPhysical Layer of the core (e.g. a bit\n\nerror or coding violation). This error\n\nis not Function-specific. STICKY."]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Reserved \\[R12\\]\n\nReserved"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bad TP Status \\[BTS\\]\n\nThis bit is set when an error is\n\ndetected in a received TLP by the\n\nData Link Layer of the core. The\n\nconditions causing this error are: (i)\n\nAn LCRC error (ii) The packet\n\nterminates with EDB symbol, but its\n\nLCRC field does not equal the\n\ninverted value of the calculated CRC.\n\nThis error is not Function-specific.\n\nSTICKY."]
    #[inline(always)]
    pub fn bts(&self) -> BtsR {
        BtsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bad DLLP Status \\[BDS\\]\n\nThis bit is set when an LCRC error is\n\ndetected in a received DLLP, and no\n\nerrors were detected by the Physical\n\nLayer. This error is not Function-\n\nspecific. STICKY."]
    #[inline(always)]
    pub fn bds(&self) -> BdsR {
        BdsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Replay Number Rollover Status \\[RNRS\\]\n\nThis bit is set when the replay count\n\nrolls over after three re-\n\ntransmissions of a TLP at the Data\n\nLink Layer of the core. This error is\n\nnot Function- specific STICKY."]
    #[inline(always)]
    pub fn rnrs(&self) -> RnrsR {
        RnrsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved \\[R13\\]\n\nReserved"]
    #[inline(always)]
    pub fn r13(&self) -> R13R {
        R13R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Status \\[RTTS\\]\n\nThis bit is set when the replay timer\n\nin the Data Link Layer of the core\n\ntimes out, causing the core to\n\nretransmit a TLP. This error is not\n\nFunction- specific. STICKY."]
    #[inline(always)]
    pub fn rtts(&self) -> RttsR {
        RttsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Advisory Non- Fatal Error Status \\[ANFES\\]\n\nThis bit is set when an uncorrectable\n\nerror occurs, which is determined to\n\nbelong to one of the special cases\n\ndescribed in Section 6.2.3.2.4 of the\n\nPCI Express 2.0 Specifications. This\n\ncauses the core to generate an\n\nERR_COR message in place of an\n\nERR_NONFATAL message. STICKY."]
    #[inline(always)]
    pub fn anfes(&self) -> AnfesR {
        AnfesR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Corrected Internal Error Status \\[CIES\\]\n\nThis bit is set when the core has\n\ndetected an internal correctable\n\nerror condition (a correctable ECC\n\nerror while reading from any of the\n\nRAMs). This bit is also set in\n\nresponse to the client signaling an\n\ninternal error through the input\n\nCORRECTABLE_ERROR_IN. This\n\nerror is not Function-specific.\n\nSTICKY."]
    #[inline(always)]
    pub fn cies(&self) -> CiesR {
        CiesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Header Log Overflow Status \\[HLOS\\]\n\nThis bit is set on a Header Log\n\nRegister overflow, that is, when the\n\nheader could not be logged in the\n\nHeader Log Register because it is\n\noccupied by a previous header.\n\nSTICKY."]
    #[inline(always)]
    pub fn hlos(&self) -> HlosR {
        HlosR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved \\[R14\\]\n\nReserved"]
    #[inline(always)]
    pub fn r14(&self) -> R14R {
        R14R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Error Status \\[RES\\]\n\nThis bit is set when an error is\n\ndetected in the receive side of the\n\nPhysical Layer of the core (e.g. a bit\n\nerror or coding violation). This error\n\nis not Function-specific. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<PciePfCorrectableErrorStatusSpec> {
        ResW::new(self, 0)
    }
    #[doc = "Bit 6 - Bad TP Status \\[BTS\\]\n\nThis bit is set when an error is\n\ndetected in a received TLP by the\n\nData Link Layer of the core. The\n\nconditions causing this error are: (i)\n\nAn LCRC error (ii) The packet\n\nterminates with EDB symbol, but its\n\nLCRC field does not equal the\n\ninverted value of the calculated CRC.\n\nThis error is not Function-specific.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn bts(&mut self) -> BtsW<PciePfCorrectableErrorStatusSpec> {
        BtsW::new(self, 6)
    }
    #[doc = "Bit 7 - Bad DLLP Status \\[BDS\\]\n\nThis bit is set when an LCRC error is\n\ndetected in a received DLLP, and no\n\nerrors were detected by the Physical\n\nLayer. This error is not Function-\n\nspecific. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn bds(&mut self) -> BdsW<PciePfCorrectableErrorStatusSpec> {
        BdsW::new(self, 7)
    }
    #[doc = "Bit 8 - Replay Number Rollover Status \\[RNRS\\]\n\nThis bit is set when the replay count\n\nrolls over after three re-\n\ntransmissions of a TLP at the Data\n\nLink Layer of the core. This error is\n\nnot Function- specific STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rnrs(&mut self) -> RnrsW<PciePfCorrectableErrorStatusSpec> {
        RnrsW::new(self, 8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Status \\[RTTS\\]\n\nThis bit is set when the replay timer\n\nin the Data Link Layer of the core\n\ntimes out, causing the core to\n\nretransmit a TLP. This error is not\n\nFunction- specific. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rtts(&mut self) -> RttsW<PciePfCorrectableErrorStatusSpec> {
        RttsW::new(self, 12)
    }
    #[doc = "Bit 13 - Advisory Non- Fatal Error Status \\[ANFES\\]\n\nThis bit is set when an uncorrectable\n\nerror occurs, which is determined to\n\nbelong to one of the special cases\n\ndescribed in Section 6.2.3.2.4 of the\n\nPCI Express 2.0 Specifications. This\n\ncauses the core to generate an\n\nERR_COR message in place of an\n\nERR_NONFATAL message. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn anfes(&mut self) -> AnfesW<PciePfCorrectableErrorStatusSpec> {
        AnfesW::new(self, 13)
    }
    #[doc = "Bit 14 - Corrected Internal Error Status \\[CIES\\]\n\nThis bit is set when the core has\n\ndetected an internal correctable\n\nerror condition (a correctable ECC\n\nerror while reading from any of the\n\nRAMs). This bit is also set in\n\nresponse to the client signaling an\n\ninternal error through the input\n\nCORRECTABLE_ERROR_IN. This\n\nerror is not Function-specific.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cies(&mut self) -> CiesW<PciePfCorrectableErrorStatusSpec> {
        CiesW::new(self, 14)
    }
    #[doc = "Bit 15 - Header Log Overflow Status \\[HLOS\\]\n\nThis bit is set on a Header Log\n\nRegister overflow, that is, when the\n\nheader could not be logged in the\n\nHeader Log Register because it is\n\noccupied by a previous header.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn hlos(&mut self) -> HlosW<PciePfCorrectableErrorStatusSpec> {
        HlosW::new(self, 15)
    }
}
#[doc = "Correctable Error Status Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_correctable_error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_correctable_error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfCorrectableErrorStatusSpec;
impl crate::RegisterSpec for PciePfCorrectableErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_correctable_error_status::R`](R) reader structure"]
impl crate::Readable for PciePfCorrectableErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_correctable_error_status::W`](W) writer structure"]
impl crate::Writable for PciePfCorrectableErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf1c1;
}
#[doc = "`reset()` method sets PCIE_PF_CORRECTABLE_ERROR_STATUS to value 0"]
impl crate::Resettable for PciePfCorrectableErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
