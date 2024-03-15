#[doc = "Register `CORRECTABLE_ERROR_STATUS` reader"]
pub type R = crate::R<CorrectableErrorStatusSpec>;
#[doc = "Register `CORRECTABLE_ERROR_STATUS` writer"]
pub type W = crate::W<CorrectableErrorStatusSpec>;
#[doc = "Field `RES` reader - Receiver Error Status \\[RES\\]
This bit is set when an error is detected in the receive side of the Physical Layer of the core (e.g. an 8b10b decode error)."]
pub type ResR = crate::BitReader;
#[doc = "Field `RES` writer - Receiver Error Status \\[RES\\]
This bit is set when an error is detected in the receive side of the Physical Layer of the core (e.g. an 8b10b decode error)."]
pub type ResW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R37` reader - Reserved \\[R37\\]
Reserved"]
pub type R37R = crate::FieldReader;
#[doc = "Field `BTS` reader - Bad TP Status \\[BTS\\]
This bit is set when an error is detected in a received TLP by the Data Link Layer of the core the conditions causing this error are (1) an LCRC error, (2) the packet terminates with EDB symbol, but its LCRC field does not equal the inverted value of the calculated CRC."]
pub type BtsR = crate::BitReader;
#[doc = "Field `BTS` writer - Bad TP Status \\[BTS\\]
This bit is set when an error is detected in a received TLP by the Data Link Layer of the core the conditions causing this error are (1) an LCRC error, (2) the packet terminates with EDB symbol, but its LCRC field does not equal the inverted value of the calculated CRC."]
pub type BtsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BDS` reader - Bad DLLP Status \\[BDS\\]
This bit is set when an LCRC error is detected in a received DLLP, and no errors were detected by the Physical Layer."]
pub type BdsR = crate::BitReader;
#[doc = "Field `BDS` writer - Bad DLLP Status \\[BDS\\]
This bit is set when an LCRC error is detected in a received DLLP, and no errors were detected by the Physical Layer."]
pub type BdsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RNRS` reader - Replay Number Rollover Status \\[RNRS\\]
This bit is set when the replay count rolls over after three re- transmissions of a TLP at the Data Link Layer of the core."]
pub type RnrsR = crate::BitReader;
#[doc = "Field `RNRS` writer - Replay Number Rollover Status \\[RNRS\\]
This bit is set when the replay count rolls over after three re- transmissions of a TLP at the Data Link Layer of the core."]
pub type RnrsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R38` reader - Reserved \\[R38\\]
Reserved"]
pub type R38R = crate::FieldReader;
#[doc = "Field `RTTS` reader - Replay Timer Timeout Status \\[RTTS\\]
This bit is set when the replay timer in the Data Link Layer of the core times out, causing the core to re- transmit a TLP."]
pub type RttsR = crate::BitReader;
#[doc = "Field `RTTS` writer - Replay Timer Timeout Status \\[RTTS\\]
This bit is set when the replay timer in the Data Link Layer of the core times out, causing the core to re- transmit a TLP."]
pub type RttsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ANES` reader - Advisory Non- Fatal Error Status \\[ANES\\]
This bit is set when an uncorrectable error occurs, which is determined to belong to one of the special cases described in the PCI Express Base Specification 2.0. This causes the core to assert the CORRECTABLE_ERROR_OUT output in place of NON_FATAL_ERROR_OUT."]
pub type AnesR = crate::BitReader;
#[doc = "Field `ANES` writer - Advisory Non- Fatal Error Status \\[ANES\\]
This bit is set when an uncorrectable error occurs, which is determined to belong to one of the special cases described in the PCI Express Base Specification 2.0. This causes the core to assert the CORRECTABLE_ERROR_OUT output in place of NON_FATAL_ERROR_OUT."]
pub type AnesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CIES` reader - Corrected Internal Error Status \\[CIES\\]
This bit is set when the core has detected an internal correctable error condition (a correctable ECC error while reading from any of the RAMs). This bit is also set in response to the client signaling an internal error through the input CORRECTABLE_ERROR_IN."]
pub type CiesR = crate::BitReader;
#[doc = "Field `CIES` writer - Corrected Internal Error Status \\[CIES\\]
This bit is set when the core has detected an internal correctable error condition (a correctable ECC error while reading from any of the RAMs). This bit is also set in response to the client signaling an internal error through the input CORRECTABLE_ERROR_IN."]
pub type CiesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HLOS` reader - Header Log Overflow Status \\[HLOS\\]
This bit is set on a Header Log Register overflow, that is, when the header could not be logged in the Header Log Register because it is occupied by a previous header."]
pub type HlosR = crate::BitReader;
#[doc = "Field `HLOS` writer - Header Log Overflow Status \\[HLOS\\]
This bit is set on a Header Log Register overflow, that is, when the header could not be logged in the Header Log Register because it is occupied by a previous header."]
pub type HlosW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R39` reader - Reserved \\[R39\\]
Reserved"]
pub type R39R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receiver Error Status \\[RES\\]
This bit is set when an error is detected in the receive side of the Physical Layer of the core (e.g. an 8b10b decode error)."]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Reserved \\[R37\\]
Reserved"]
    #[inline(always)]
    pub fn r37(&self) -> R37R {
        R37R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bad TP Status \\[BTS\\]
This bit is set when an error is detected in a received TLP by the Data Link Layer of the core the conditions causing this error are (1) an LCRC error, (2) the packet terminates with EDB symbol, but its LCRC field does not equal the inverted value of the calculated CRC."]
    #[inline(always)]
    pub fn bts(&self) -> BtsR {
        BtsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bad DLLP Status \\[BDS\\]
This bit is set when an LCRC error is detected in a received DLLP, and no errors were detected by the Physical Layer."]
    #[inline(always)]
    pub fn bds(&self) -> BdsR {
        BdsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Replay Number Rollover Status \\[RNRS\\]
This bit is set when the replay count rolls over after three re- transmissions of a TLP at the Data Link Layer of the core."]
    #[inline(always)]
    pub fn rnrs(&self) -> RnrsR {
        RnrsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved \\[R38\\]
Reserved"]
    #[inline(always)]
    pub fn r38(&self) -> R38R {
        R38R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Status \\[RTTS\\]
This bit is set when the replay timer in the Data Link Layer of the core times out, causing the core to re- transmit a TLP."]
    #[inline(always)]
    pub fn rtts(&self) -> RttsR {
        RttsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Advisory Non- Fatal Error Status \\[ANES\\]
This bit is set when an uncorrectable error occurs, which is determined to belong to one of the special cases described in the PCI Express Base Specification 2.0. This causes the core to assert the CORRECTABLE_ERROR_OUT output in place of NON_FATAL_ERROR_OUT."]
    #[inline(always)]
    pub fn anes(&self) -> AnesR {
        AnesR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Corrected Internal Error Status \\[CIES\\]
This bit is set when the core has detected an internal correctable error condition (a correctable ECC error while reading from any of the RAMs). This bit is also set in response to the client signaling an internal error through the input CORRECTABLE_ERROR_IN."]
    #[inline(always)]
    pub fn cies(&self) -> CiesR {
        CiesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Header Log Overflow Status \\[HLOS\\]
This bit is set on a Header Log Register overflow, that is, when the header could not be logged in the Header Log Register because it is occupied by a previous header."]
    #[inline(always)]
    pub fn hlos(&self) -> HlosR {
        HlosR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved \\[R39\\]
Reserved"]
    #[inline(always)]
    pub fn r39(&self) -> R39R {
        R39R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Error Status \\[RES\\]
This bit is set when an error is detected in the receive side of the Physical Layer of the core (e.g. an 8b10b decode error)."]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<CorrectableErrorStatusSpec> {
        ResW::new(self, 0)
    }
    #[doc = "Bit 6 - Bad TP Status \\[BTS\\]
This bit is set when an error is detected in a received TLP by the Data Link Layer of the core the conditions causing this error are (1) an LCRC error, (2) the packet terminates with EDB symbol, but its LCRC field does not equal the inverted value of the calculated CRC."]
    #[inline(always)]
    #[must_use]
    pub fn bts(&mut self) -> BtsW<CorrectableErrorStatusSpec> {
        BtsW::new(self, 6)
    }
    #[doc = "Bit 7 - Bad DLLP Status \\[BDS\\]
This bit is set when an LCRC error is detected in a received DLLP, and no errors were detected by the Physical Layer."]
    #[inline(always)]
    #[must_use]
    pub fn bds(&mut self) -> BdsW<CorrectableErrorStatusSpec> {
        BdsW::new(self, 7)
    }
    #[doc = "Bit 8 - Replay Number Rollover Status \\[RNRS\\]
This bit is set when the replay count rolls over after three re- transmissions of a TLP at the Data Link Layer of the core."]
    #[inline(always)]
    #[must_use]
    pub fn rnrs(&mut self) -> RnrsW<CorrectableErrorStatusSpec> {
        RnrsW::new(self, 8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Status \\[RTTS\\]
This bit is set when the replay timer in the Data Link Layer of the core times out, causing the core to re- transmit a TLP."]
    #[inline(always)]
    #[must_use]
    pub fn rtts(&mut self) -> RttsW<CorrectableErrorStatusSpec> {
        RttsW::new(self, 12)
    }
    #[doc = "Bit 13 - Advisory Non- Fatal Error Status \\[ANES\\]
This bit is set when an uncorrectable error occurs, which is determined to belong to one of the special cases described in the PCI Express Base Specification 2.0. This causes the core to assert the CORRECTABLE_ERROR_OUT output in place of NON_FATAL_ERROR_OUT."]
    #[inline(always)]
    #[must_use]
    pub fn anes(&mut self) -> AnesW<CorrectableErrorStatusSpec> {
        AnesW::new(self, 13)
    }
    #[doc = "Bit 14 - Corrected Internal Error Status \\[CIES\\]
This bit is set when the core has detected an internal correctable error condition (a correctable ECC error while reading from any of the RAMs). This bit is also set in response to the client signaling an internal error through the input CORRECTABLE_ERROR_IN."]
    #[inline(always)]
    #[must_use]
    pub fn cies(&mut self) -> CiesW<CorrectableErrorStatusSpec> {
        CiesW::new(self, 14)
    }
    #[doc = "Bit 15 - Header Log Overflow Status \\[HLOS\\]
This bit is set on a Header Log Register overflow, that is, when the header could not be logged in the Header Log Register because it is occupied by a previous header."]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf1c1;
}
#[doc = "`reset()` method sets CORRECTABLE_ERROR_STATUS to value 0"]
impl crate::Resettable for CorrectableErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
