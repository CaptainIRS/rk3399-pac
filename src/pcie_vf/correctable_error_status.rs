#[doc = "Register `CORRECTABLE_ERROR_STATUS` reader"]
pub type R = crate::R<CorrectableErrorStatusSpec>;
#[doc = "Register `CORRECTABLE_ERROR_STATUS` writer"]
pub type W = crate::W<CorrectableErrorStatusSpec>;
#[doc = "Field `RES` reader - Receiver Error Status \\[RES\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type ResR = crate::BitReader;
#[doc = "Field `R12` reader - Reserved \\[R12\\]
Reserved"]
pub type R12R = crate::FieldReader;
#[doc = "Field `BTPS` reader - Bad TP Status \\[BTPS\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type BtpsR = crate::BitReader;
#[doc = "Field `BDS` reader - Bad DLLP Status \\[BDS\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type BdsR = crate::BitReader;
#[doc = "Field `RNRS` reader - Replay Number Rollover Status \\[RNRS\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type RnrsR = crate::BitReader;
#[doc = "Field `R13` reader - Reserved \\[R13\\]
Reserved"]
pub type R13R = crate::FieldReader;
#[doc = "Field `RTTS` reader - Replay Timer Timeout Status \\[RTTS\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type RttsR = crate::BitReader;
#[doc = "Field `ANFES` reader - Advisory Non- Fatal Error Status \\[ANFES\\]
This bit is set when an uncorrectable error occurs, which is determined to belong to one of the special cases described in Section 6.2.3.2.4 of the PCI Express 2.0 Specifications. This causes the core to generate an ERR_COR message in place of an ERR_NONFATAL message. STICKY."]
pub type AnfesR = crate::BitReader;
#[doc = "Field `ANFES` writer - Advisory Non- Fatal Error Status \\[ANFES\\]
This bit is set when an uncorrectable error occurs, which is determined to belong to one of the special cases described in Section 6.2.3.2.4 of the PCI Express 2.0 Specifications. This causes the core to generate an ERR_COR message in place of an ERR_NONFATAL message. STICKY."]
pub type AnfesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIES` reader - Corrected Internal Error Status \\[CIES\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type CiesR = crate::BitReader;
#[doc = "Field `HLOS` reader - Header Log Overflow Status \\[HLOS\\]
This bit is set on a Header Log Register overflow, that is, when the header could not be logged in the Header Log Register because it is occupied by a previous header. STICKY."]
pub type HlosR = crate::BitReader;
#[doc = "Field `HLOS` writer - Header Log Overflow Status \\[HLOS\\]
This bit is set on a Header Log Register overflow, that is, when the header could not be logged in the Header Log Register because it is occupied by a previous header. STICKY."]
pub type HlosW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R14` reader - Reserved \\[R14\\]
Reserved"]
pub type R14R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receiver Error Status \\[RES\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Reserved \\[R12\\]
Reserved"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bad TP Status \\[BTPS\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn btps(&self) -> BtpsR {
        BtpsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bad DLLP Status \\[BDS\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn bds(&self) -> BdsR {
        BdsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Replay Number Rollover Status \\[RNRS\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn rnrs(&self) -> RnrsR {
        RnrsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved \\[R13\\]
Reserved"]
    #[inline(always)]
    pub fn r13(&self) -> R13R {
        R13R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Status \\[RTTS\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn rtts(&self) -> RttsR {
        RttsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Advisory Non- Fatal Error Status \\[ANFES\\]
This bit is set when an uncorrectable error occurs, which is determined to belong to one of the special cases described in Section 6.2.3.2.4 of the PCI Express 2.0 Specifications. This causes the core to generate an ERR_COR message in place of an ERR_NONFATAL message. STICKY."]
    #[inline(always)]
    pub fn anfes(&self) -> AnfesR {
        AnfesR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Corrected Internal Error Status \\[CIES\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn cies(&self) -> CiesR {
        CiesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Header Log Overflow Status \\[HLOS\\]
This bit is set on a Header Log Register overflow, that is, when the header could not be logged in the Header Log Register because it is occupied by a previous header. STICKY."]
    #[inline(always)]
    pub fn hlos(&self) -> HlosR {
        HlosR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved \\[R14\\]
Reserved"]
    #[inline(always)]
    pub fn r14(&self) -> R14R {
        R14R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 13 - Advisory Non- Fatal Error Status \\[ANFES\\]
This bit is set when an uncorrectable error occurs, which is determined to belong to one of the special cases described in Section 6.2.3.2.4 of the PCI Express 2.0 Specifications. This causes the core to generate an ERR_COR message in place of an ERR_NONFATAL message. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn anfes(&mut self) -> AnfesW<CorrectableErrorStatusSpec> {
        AnfesW::new(self, 13)
    }
    #[doc = "Bit 15 - Header Log Overflow Status \\[HLOS\\]
This bit is set on a Header Log Register overflow, that is, when the header could not be logged in the Header Log Register because it is occupied by a previous header. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn hlos(&mut self) -> HlosW<CorrectableErrorStatusSpec> {
        HlosW::new(self, 15)
    }
}
#[doc = "Correctable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`correctable_error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`correctable_error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CorrectableErrorStatusSpec;
impl crate::RegisterSpec for CorrectableErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`correctable_error_status::R`](R) reader structure"]
impl crate::Readable for CorrectableErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`correctable_error_status::W`](W) writer structure"]
impl crate::Writable for CorrectableErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xa000;
}
#[doc = "`reset()` method sets CORRECTABLE_ERROR_STATUS to value 0"]
impl crate::Resettable for CorrectableErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
