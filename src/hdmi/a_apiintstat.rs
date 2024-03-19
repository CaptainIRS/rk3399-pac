#[doc = "Register `A_APIINTSTAT` reader"]
pub type R = crate::R<AApiintstatSpec>;
#[doc = "Field `KSVACCESSINT` reader - Notifies that the KSV Attr as been guaranteed for\n\nRead-Write access."]
pub type KsvaccessintR = crate::BitReader;
#[doc = "Field `KSVSHA1CALCINT` reader - Notifies that the HDCP13TCTRL controller as\n\nupdated a KSV list in memory that needs to be\n\nSHA1 verified."]
pub type Ksvsha1calcintR = crate::BitReader;
#[doc = "Field `KEEPOUTERRORINT` reader - Notifies that during the keep out window, the\n\nctlout\\[3:0\\]
bus was used besides control period."]
pub type KeepouterrorintR = crate::BitReader;
#[doc = "Field `LOSTARBITRATION` reader - Notifies that the I2C lost the arbitration to\n\ncommunicate. Another master gained arbitration."]
pub type LostarbitrationR = crate::BitReader;
#[doc = "Field `I2CNACK` reader - Notifies that the I2C received a NACK from slave\n\ndevice."]
pub type I2cnackR = crate::BitReader;
#[doc = "Field `KSVSHA1CALCDONEINT` reader - Notifies that the HDCP13TCTRL controller SHA1\n\nverification has been done. The status ready to be\n\nread."]
pub type Ksvsha1calcdoneintR = crate::BitReader;
#[doc = "Field `HDCP_FAILED` reader - Notifies that the HDCP authentication process was\n\nfailed."]
pub type HdcpFailedR = crate::BitReader;
#[doc = "Field `HDCP_ENGAGED` reader - Notifies that the HDCP authentication process was\n\nsuccessful"]
pub type HdcpEngagedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Notifies that the KSV Attr as been guaranteed for\n\nRead-Write access."]
    #[inline(always)]
    pub fn ksvaccessint(&self) -> KsvaccessintR {
        KsvaccessintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Notifies that the HDCP13TCTRL controller as\n\nupdated a KSV list in memory that needs to be\n\nSHA1 verified."]
    #[inline(always)]
    pub fn ksvsha1calcint(&self) -> Ksvsha1calcintR {
        Ksvsha1calcintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Notifies that during the keep out window, the\n\nctlout\\[3:0\\]
bus was used besides control period."]
    #[inline(always)]
    pub fn keepouterrorint(&self) -> KeepouterrorintR {
        KeepouterrorintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Notifies that the I2C lost the arbitration to\n\ncommunicate. Another master gained arbitration."]
    #[inline(always)]
    pub fn lostarbitration(&self) -> LostarbitrationR {
        LostarbitrationR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Notifies that the I2C received a NACK from slave\n\ndevice."]
    #[inline(always)]
    pub fn i2cnack(&self) -> I2cnackR {
        I2cnackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Notifies that the HDCP13TCTRL controller SHA1\n\nverification has been done. The status ready to be\n\nread."]
    #[inline(always)]
    pub fn ksvsha1calcdoneint(&self) -> Ksvsha1calcdoneintR {
        Ksvsha1calcdoneintR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Notifies that the HDCP authentication process was\n\nfailed."]
    #[inline(always)]
    pub fn hdcp_failed(&self) -> HdcpFailedR {
        HdcpFailedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Notifies that the HDCP authentication process was\n\nsuccessful"]
    #[inline(always)]
    pub fn hdcp_engaged(&self) -> HdcpEngagedR {
        HdcpEngagedR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "HDCP Interrupt Status Register\n\nRead only register, reports the interruption which caused the activation of the interruption\n\noutput pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_apiintstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AApiintstatSpec;
impl crate::RegisterSpec for AApiintstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_apiintstat::R`](R) reader structure"]
impl crate::Readable for AApiintstatSpec {}
#[doc = "`reset()` method sets A_APIINTSTAT to value 0"]
impl crate::Resettable for AApiintstatSpec {
    const RESET_VALUE: u8 = 0;
}
