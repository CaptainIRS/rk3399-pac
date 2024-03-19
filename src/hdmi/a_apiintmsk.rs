#[doc = "Register `A_APIINTMSK` reader"]
pub type R = crate::R<AApiintmskSpec>;
#[doc = "Register `A_APIINTMSK` writer"]
pub type W = crate::W<AApiintmskSpec>;
#[doc = "Field `KSVACCESSINT` reader - Masks the interruption related to KSV Attr grant for\n\nRead-Write access."]
pub type KsvaccessintR = crate::BitReader;
#[doc = "Field `KSVACCESSINT` writer - Masks the interruption related to KSV Attr grant for\n\nRead-Write access."]
pub type KsvaccessintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KSVSHA1CALCINT` reader - Masks the interruption related to KSV list update in\n\nmemory that needs to be SHA1 verified. Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality."]
pub type Ksvsha1calcintR = crate::BitReader;
#[doc = "Field `KSVSHA1CALCINT` writer - Masks the interruption related to KSV list update in\n\nmemory that needs to be SHA1 verified. Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality."]
pub type Ksvsha1calcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEPOUTERRORINT` reader - Masks the interruption related to keep out window\n\nerror."]
pub type KeepouterrorintR = crate::BitReader;
#[doc = "Field `KEEPOUTERRORINT` writer - Masks the interruption related to keep out window\n\nerror."]
pub type KeepouterrorintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOSTARBITRATION` reader - Masks the interruption related to I2C arbitration\n\nlost."]
pub type LostarbitrationR = crate::BitReader;
#[doc = "Field `LOSTARBITRATION` writer - Masks the interruption related to I2C arbitration\n\nlost."]
pub type LostarbitrationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CNACK` reader - Masks the interruption related to I2C NACK\n\nreception."]
pub type I2cnackR = crate::BitReader;
#[doc = "Field `I2CNACK` writer - Masks the interruption related to I2C NACK\n\nreception."]
pub type I2cnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KSVSHA1CALCDONEINT` reader - Masks the interruption related to SHA1 verification\n\nhas been done Otherwise, this field is a \"spare\" bit\n\nwith no associated functionality."]
pub type Ksvsha1calcdoneintR = crate::BitReader;
#[doc = "Field `KSVSHA1CALCDONEINT` writer - Masks the interruption related to SHA1 verification\n\nhas been done Otherwise, this field is a \"spare\" bit\n\nwith no associated functionality."]
pub type Ksvsha1calcdoneintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDCP_FAILED` reader - Masks the interruption related to HDCP\n\nauthentication process failed."]
pub type HdcpFailedR = crate::BitReader;
#[doc = "Field `HDCP_FAILED` writer - Masks the interruption related to HDCP\n\nauthentication process failed."]
pub type HdcpFailedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDCP_ENGAGED` reader - Masks the interruption related to HDCP\n\nauthentication process successful."]
pub type HdcpEngagedR = crate::BitReader;
#[doc = "Field `HDCP_ENGAGED` writer - Masks the interruption related to HDCP\n\nauthentication process successful."]
pub type HdcpEngagedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Masks the interruption related to KSV Attr grant for\n\nRead-Write access."]
    #[inline(always)]
    pub fn ksvaccessint(&self) -> KsvaccessintR {
        KsvaccessintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masks the interruption related to KSV list update in\n\nmemory that needs to be SHA1 verified. Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality."]
    #[inline(always)]
    pub fn ksvsha1calcint(&self) -> Ksvsha1calcintR {
        Ksvsha1calcintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masks the interruption related to keep out window\n\nerror."]
    #[inline(always)]
    pub fn keepouterrorint(&self) -> KeepouterrorintR {
        KeepouterrorintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masks the interruption related to I2C arbitration\n\nlost."]
    #[inline(always)]
    pub fn lostarbitration(&self) -> LostarbitrationR {
        LostarbitrationR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masks the interruption related to I2C NACK\n\nreception."]
    #[inline(always)]
    pub fn i2cnack(&self) -> I2cnackR {
        I2cnackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masks the interruption related to SHA1 verification\n\nhas been done Otherwise, this field is a \"spare\" bit\n\nwith no associated functionality."]
    #[inline(always)]
    pub fn ksvsha1calcdoneint(&self) -> Ksvsha1calcdoneintR {
        Ksvsha1calcdoneintR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masks the interruption related to HDCP\n\nauthentication process failed."]
    #[inline(always)]
    pub fn hdcp_failed(&self) -> HdcpFailedR {
        HdcpFailedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masks the interruption related to HDCP\n\nauthentication process successful."]
    #[inline(always)]
    pub fn hdcp_engaged(&self) -> HdcpEngagedR {
        HdcpEngagedR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masks the interruption related to KSV Attr grant for\n\nRead-Write access."]
    #[inline(always)]
    #[must_use]
    pub fn ksvaccessint(&mut self) -> KsvaccessintW<AApiintmskSpec> {
        KsvaccessintW::new(self, 0)
    }
    #[doc = "Bit 1 - Masks the interruption related to KSV list update in\n\nmemory that needs to be SHA1 verified. Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality."]
    #[inline(always)]
    #[must_use]
    pub fn ksvsha1calcint(&mut self) -> Ksvsha1calcintW<AApiintmskSpec> {
        Ksvsha1calcintW::new(self, 1)
    }
    #[doc = "Bit 2 - Masks the interruption related to keep out window\n\nerror."]
    #[inline(always)]
    #[must_use]
    pub fn keepouterrorint(&mut self) -> KeepouterrorintW<AApiintmskSpec> {
        KeepouterrorintW::new(self, 2)
    }
    #[doc = "Bit 3 - Masks the interruption related to I2C arbitration\n\nlost."]
    #[inline(always)]
    #[must_use]
    pub fn lostarbitration(&mut self) -> LostarbitrationW<AApiintmskSpec> {
        LostarbitrationW::new(self, 3)
    }
    #[doc = "Bit 4 - Masks the interruption related to I2C NACK\n\nreception."]
    #[inline(always)]
    #[must_use]
    pub fn i2cnack(&mut self) -> I2cnackW<AApiintmskSpec> {
        I2cnackW::new(self, 4)
    }
    #[doc = "Bit 5 - Masks the interruption related to SHA1 verification\n\nhas been done Otherwise, this field is a \"spare\" bit\n\nwith no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn ksvsha1calcdoneint(&mut self) -> Ksvsha1calcdoneintW<AApiintmskSpec> {
        Ksvsha1calcdoneintW::new(self, 5)
    }
    #[doc = "Bit 6 - Masks the interruption related to HDCP\n\nauthentication process failed."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_failed(&mut self) -> HdcpFailedW<AApiintmskSpec> {
        HdcpFailedW::new(self, 6)
    }
    #[doc = "Bit 7 - Masks the interruption related to HDCP\n\nauthentication process successful."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_engaged(&mut self) -> HdcpEngagedW<AApiintmskSpec> {
        HdcpEngagedW::new(self, 7)
    }
}
#[doc = "HDCP Interrupt Mask Register\n\nThe configuration of this register mask a given setup of interruption, disabling them from\n\ngenerating interruption pulses in the interruption output pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_apiintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_apiintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AApiintmskSpec;
impl crate::RegisterSpec for AApiintmskSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_apiintmsk::R`](R) reader structure"]
impl crate::Readable for AApiintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`a_apiintmsk::W`](W) writer structure"]
impl crate::Writable for AApiintmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets A_APIINTMSK to value 0"]
impl crate::Resettable for AApiintmskSpec {
    const RESET_VALUE: u8 = 0;
}
