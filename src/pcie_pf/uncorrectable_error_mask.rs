#[doc = "Register `UNCORRECTABLE_ERROR_MASK` reader"]
pub type R = crate::R<UncorrectableErrorMaskSpec>;
#[doc = "Register `UNCORRECTABLE_ERROR_MASK` writer"]
pub type W = crate::W<UncorrectableErrorMaskSpec>;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader;
#[doc = "Field `DLPEM` reader - Data Link Protocol Error Mask \\[DLPEM\\]
This bit is set to mask the reporting of Data Link Protocol Errors. STICKY."]
pub type DlpemR = crate::BitReader;
#[doc = "Field `DLPEM` writer - Data Link Protocol Error Mask \\[DLPEM\\]
This bit is set to mask the reporting of Data Link Protocol Errors. STICKY."]
pub type DlpemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5` reader - Reserved \\[R5\\]
Reserved"]
pub type R5R = crate::FieldReader;
#[doc = "Field `PTM` reader - Poisoned TLP Mask \\[PTM\\]
This bit is set to mask the reporting of a Poisoned TLP. STICKY."]
pub type PtmR = crate::BitReader;
#[doc = "Field `PTM` writer - Poisoned TLP Mask \\[PTM\\]
This bit is set to mask the reporting of a Poisoned TLP. STICKY."]
pub type PtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCPEM` reader - Flow Control Protocol Error Mask \\[FCPEM\\]
This bit is set to mask the reporting of Flow Control Protocol Errors. STICKY."]
pub type FcpemR = crate::BitReader;
#[doc = "Field `FCPEM` writer - Flow Control Protocol Error Mask \\[FCPEM\\]
This bit is set to mask the reporting of Flow Control Protocol Errors. STICKY."]
pub type FcpemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTM` reader - Completion Timeout Mask \\[CTM\\]
This bit is set to mask the reporting of Completion Timeouts. STICKY."]
pub type CtmR = crate::BitReader;
#[doc = "Field `CTM` writer - Completion Timeout Mask \\[CTM\\]
This bit is set to mask the reporting of Completion Timeouts. STICKY."]
pub type CtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM` reader - Completer Abort Mask \\[CAM\\]
This bit is set to mask the reporting of the core sending a Completer Abort. STICKY."]
pub type CamR = crate::BitReader;
#[doc = "Field `CAM` writer - Completer Abort Mask \\[CAM\\]
This bit is set to mask the reporting of the core sending a Completer Abort. STICKY."]
pub type CamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCM` reader - Unexpected Completion Mask \\[UCM\\]
This bit is set to mask the reporting of unexpected Completions received by the core. STICKY."]
pub type UcmR = crate::BitReader;
#[doc = "Field `UCM` writer - Unexpected Completion Mask \\[UCM\\]
This bit is set to mask the reporting of unexpected Completions received by the core. STICKY."]
pub type UcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` reader - Receiver Overflow Mask \\[ROM\\]
This bit is set to mask the reporting of violations of receive credit. STICKY."]
pub type RomR = crate::BitReader;
#[doc = "Field `ROM` writer - Receiver Overflow Mask \\[ROM\\]
This bit is set to mask the reporting of violations of receive credit. STICKY."]
pub type RomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTM` reader - Malformed TLP Mask \\[MTM\\]
This bit is set to mask the reporting of malformed TLPs received from the link. STICKY."]
pub type MtmR = crate::BitReader;
#[doc = "Field `MTM` writer - Malformed TLP Mask \\[MTM\\]
This bit is set to mask the reporting of malformed TLPs received from the link. STICKY."]
pub type MtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEM` reader - ECRC Error Mask \\[EEM\\]
This bit is set to mask the reporting of ECRC errors. STICKY."]
pub type EemR = crate::BitReader;
#[doc = "Field `EEM` writer - ECRC Error Mask \\[EEM\\]
This bit is set to mask the reporting of ECRC errors. STICKY."]
pub type EemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UREM` reader - Unsupported Request Error Mask \\[UREM\\]
This bit is set to mask the reporting of unexpected requests received from the link. STICKY."]
pub type UremR = crate::BitReader;
#[doc = "Field `UREM` writer - Unsupported Request Error Mask \\[UREM\\]
This bit is set to mask the reporting of unexpected requests received from the link. STICKY."]
pub type UremW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R6` reader - Reserved \\[R6\\]
Reserved"]
pub type R6R = crate::BitReader;
#[doc = "Field `UIEM` reader - Uncorrectable Internal Error Mask \\[UIEM\\]
This bit is set to mask the reporting of internal errors. STICKY."]
pub type UiemR = crate::BitReader;
#[doc = "Field `UIEM` writer - Uncorrectable Internal Error Mask \\[UIEM\\]
This bit is set to mask the reporting of internal errors. STICKY."]
pub type UiemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R7` reader - Reserved \\[R7\\]
Reserved"]
pub type R7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Data Link Protocol Error Mask \\[DLPEM\\]
This bit is set to mask the reporting of Data Link Protocol Errors. STICKY."]
    #[inline(always)]
    pub fn dlpem(&self) -> DlpemR {
        DlpemR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - Reserved \\[R5\\]
Reserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Poisoned TLP Mask \\[PTM\\]
This bit is set to mask the reporting of a Poisoned TLP. STICKY."]
    #[inline(always)]
    pub fn ptm(&self) -> PtmR {
        PtmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Mask \\[FCPEM\\]
This bit is set to mask the reporting of Flow Control Protocol Errors. STICKY."]
    #[inline(always)]
    pub fn fcpem(&self) -> FcpemR {
        FcpemR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Completion Timeout Mask \\[CTM\\]
This bit is set to mask the reporting of Completion Timeouts. STICKY."]
    #[inline(always)]
    pub fn ctm(&self) -> CtmR {
        CtmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Completer Abort Mask \\[CAM\\]
This bit is set to mask the reporting of the core sending a Completer Abort. STICKY."]
    #[inline(always)]
    pub fn cam(&self) -> CamR {
        CamR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Unexpected Completion Mask \\[UCM\\]
This bit is set to mask the reporting of unexpected Completions received by the core. STICKY."]
    #[inline(always)]
    pub fn ucm(&self) -> UcmR {
        UcmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Overflow Mask \\[ROM\\]
This bit is set to mask the reporting of violations of receive credit. STICKY."]
    #[inline(always)]
    pub fn rom(&self) -> RomR {
        RomR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Malformed TLP Mask \\[MTM\\]
This bit is set to mask the reporting of malformed TLPs received from the link. STICKY."]
    #[inline(always)]
    pub fn mtm(&self) -> MtmR {
        MtmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ECRC Error Mask \\[EEM\\]
This bit is set to mask the reporting of ECRC errors. STICKY."]
    #[inline(always)]
    pub fn eem(&self) -> EemR {
        EemR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Unsupported Request Error Mask \\[UREM\\]
This bit is set to mask the reporting of unexpected requests received from the link. STICKY."]
    #[inline(always)]
    pub fn urem(&self) -> UremR {
        UremR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved \\[R6\\]
Reserved"]
    #[inline(always)]
    pub fn r6(&self) -> R6R {
        R6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Mask \\[UIEM\\]
This bit is set to mask the reporting of internal errors. STICKY."]
    #[inline(always)]
    pub fn uiem(&self) -> UiemR {
        UiemR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved \\[R7\\]
Reserved"]
    #[inline(always)]
    pub fn r7(&self) -> R7R {
        R7R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Data Link Protocol Error Mask \\[DLPEM\\]
This bit is set to mask the reporting of Data Link Protocol Errors. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn dlpem(&mut self) -> DlpemW<UncorrectableErrorMaskSpec> {
        DlpemW::new(self, 4)
    }
    #[doc = "Bit 12 - Poisoned TLP Mask \\[PTM\\]
This bit is set to mask the reporting of a Poisoned TLP. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ptm(&mut self) -> PtmW<UncorrectableErrorMaskSpec> {
        PtmW::new(self, 12)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Mask \\[FCPEM\\]
This bit is set to mask the reporting of Flow Control Protocol Errors. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn fcpem(&mut self) -> FcpemW<UncorrectableErrorMaskSpec> {
        FcpemW::new(self, 13)
    }
    #[doc = "Bit 14 - Completion Timeout Mask \\[CTM\\]
This bit is set to mask the reporting of Completion Timeouts. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ctm(&mut self) -> CtmW<UncorrectableErrorMaskSpec> {
        CtmW::new(self, 14)
    }
    #[doc = "Bit 15 - Completer Abort Mask \\[CAM\\]
This bit is set to mask the reporting of the core sending a Completer Abort. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cam(&mut self) -> CamW<UncorrectableErrorMaskSpec> {
        CamW::new(self, 15)
    }
    #[doc = "Bit 16 - Unexpected Completion Mask \\[UCM\\]
This bit is set to mask the reporting of unexpected Completions received by the core. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ucm(&mut self) -> UcmW<UncorrectableErrorMaskSpec> {
        UcmW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Overflow Mask \\[ROM\\]
This bit is set to mask the reporting of violations of receive credit. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> RomW<UncorrectableErrorMaskSpec> {
        RomW::new(self, 17)
    }
    #[doc = "Bit 18 - Malformed TLP Mask \\[MTM\\]
This bit is set to mask the reporting of malformed TLPs received from the link. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn mtm(&mut self) -> MtmW<UncorrectableErrorMaskSpec> {
        MtmW::new(self, 18)
    }
    #[doc = "Bit 19 - ECRC Error Mask \\[EEM\\]
This bit is set to mask the reporting of ECRC errors. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn eem(&mut self) -> EemW<UncorrectableErrorMaskSpec> {
        EemW::new(self, 19)
    }
    #[doc = "Bit 20 - Unsupported Request Error Mask \\[UREM\\]
This bit is set to mask the reporting of unexpected requests received from the link. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn urem(&mut self) -> UremW<UncorrectableErrorMaskSpec> {
        UremW::new(self, 20)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Mask \\[UIEM\\]
This bit is set to mask the reporting of internal errors. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn uiem(&mut self) -> UiemW<UncorrectableErrorMaskSpec> {
        UiemW::new(self, 22)
    }
}
#[doc = "Uncorrectable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uncorrectable_error_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UncorrectableErrorMaskSpec;
impl crate::RegisterSpec for UncorrectableErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uncorrectable_error_mask::R`](R) reader structure"]
impl crate::Readable for UncorrectableErrorMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`uncorrectable_error_mask::W`](W) writer structure"]
impl crate::Writable for UncorrectableErrorMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UNCORRECTABLE_ERROR_MASK to value 0x0040_0000"]
impl crate::Resettable for UncorrectableErrorMaskSpec {
    const RESET_VALUE: u32 = 0x0040_0000;
}
