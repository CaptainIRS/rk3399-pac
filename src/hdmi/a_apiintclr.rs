#[doc = "Register `A_APIINTCLR` writer"]
pub type W = crate::W<AApiintclrSpec>;
#[doc = "Field `KSVACCESSINT` writer - Clears the interruption related to KSV Attr grant for Read-Write access."]
pub type KsvaccessintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KSVSHA1CALCINT` writer - Clears the interruption related to KSV list update in memory that needs to be SHA1 verified."]
pub type Ksvsha1calcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEPOUTERRORINT` writer - Clears the interruption related to keep out window error."]
pub type KeepouterrorintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOSTARBITRATION` writer - Clears the interruption related to I2C arbitration lost."]
pub type LostarbitrationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CNACK` writer - Clears the interruption related to I2C NACK reception."]
pub type I2cnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KSVSHA1CALCDONEINT` writer - Clears the interruption related to SHA1 verification has been done"]
pub type Ksvsha1calcdoneintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDCP_FAILED` writer - Clears the interruption related to HDCP authentication process failed."]
pub type HdcpFailedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDCP_ENGAGED` writer - Clears the interruption related to HDCP authentication process successful."]
pub type HdcpEngagedW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clears the interruption related to KSV Attr grant for Read-Write access."]
    #[inline(always)]
    #[must_use]
    pub fn ksvaccessint(&mut self) -> KsvaccessintW<AApiintclrSpec> {
        KsvaccessintW::new(self, 0)
    }
    #[doc = "Bit 1 - Clears the interruption related to KSV list update in memory that needs to be SHA1 verified."]
    #[inline(always)]
    #[must_use]
    pub fn ksvsha1calcint(&mut self) -> Ksvsha1calcintW<AApiintclrSpec> {
        Ksvsha1calcintW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears the interruption related to keep out window error."]
    #[inline(always)]
    #[must_use]
    pub fn keepouterrorint(&mut self) -> KeepouterrorintW<AApiintclrSpec> {
        KeepouterrorintW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears the interruption related to I2C arbitration lost."]
    #[inline(always)]
    #[must_use]
    pub fn lostarbitration(&mut self) -> LostarbitrationW<AApiintclrSpec> {
        LostarbitrationW::new(self, 3)
    }
    #[doc = "Bit 4 - Clears the interruption related to I2C NACK reception."]
    #[inline(always)]
    #[must_use]
    pub fn i2cnack(&mut self) -> I2cnackW<AApiintclrSpec> {
        I2cnackW::new(self, 4)
    }
    #[doc = "Bit 5 - Clears the interruption related to SHA1 verification has been done"]
    #[inline(always)]
    #[must_use]
    pub fn ksvsha1calcdoneint(&mut self) -> Ksvsha1calcdoneintW<AApiintclrSpec> {
        Ksvsha1calcdoneintW::new(self, 5)
    }
    #[doc = "Bit 6 - Clears the interruption related to HDCP authentication process failed."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_failed(&mut self) -> HdcpFailedW<AApiintclrSpec> {
        HdcpFailedW::new(self, 6)
    }
    #[doc = "Bit 7 - Clears the interruption related to HDCP authentication process successful."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_engaged(&mut self) -> HdcpEngagedW<AApiintclrSpec> {
        HdcpEngagedW::new(self, 7)
    }
}
#[doc = "Clears the interruption related to KSV Attr grant for Read-Write access.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_apiintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AApiintclrSpec;
impl crate::RegisterSpec for AApiintclrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`a_apiintclr::W`](W) writer structure"]
impl crate::Writable for AApiintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets A_APIINTCLR to value 0"]
impl crate::Resettable for AApiintclrSpec {
    const RESET_VALUE: u8 = 0;
}
