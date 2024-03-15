#[doc = "Register `A_HDCPCFG1` reader"]
pub type R = crate::R<AHdcpcfg1Spec>;
#[doc = "Register `A_HDCPCFG1` writer"]
pub type W = crate::W<AHdcpcfg1Spec>;
#[doc = "Field `SWRESET` reader - Software reset signal, active by writing a zero and auto cleared to 1 in the following cycle."]
pub type SwresetR = crate::BitReader;
#[doc = "Field `SWRESET` writer - Software reset signal, active by writing a zero and auto cleared to 1 in the following cycle."]
pub type SwresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCRYPTIONDISABLE` reader - Disable encryption without losing authentication"]
pub type EncryptiondisableR = crate::BitReader;
#[doc = "Field `ENCRYPTIONDISABLE` writer - Disable encryption without losing authentication"]
pub type EncryptiondisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH2UPSHFTENC` reader - Enables the encoding of packet header in the tmdsch0 bit\\[0\\]
with cipher\\[2\\]
instead of the tmdsch0 bit\\[2\\]
Note: This bit must always be set to 1 for all PHYs."]
pub type Ph2upshftencR = crate::BitReader;
#[doc = "Field `PH2UPSHFTENC` writer - Enables the encoding of packet header in the tmdsch0 bit\\[0\\]
with cipher\\[2\\]
instead of the tmdsch0 bit\\[2\\]
Note: This bit must always be set to 1 for all PHYs."]
pub type Ph2upshftencW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISSHA1CHECK` reader - Disables the request to the API processor to verify the SHA1 message digest of a received KSV List"]
pub type Dissha1checkR = crate::BitReader;
#[doc = "Field `DISSHA1CHECK` writer - Disables the request to the API processor to verify the SHA1 message digest of a received KSV List"]
pub type Dissha1checkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Lock the HDCP bypass and encryption disable mechanisms:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdcpLock {
    #[doc = "0: You can still write to the bit bypencryption of A_HDCPCFG0 or encryptiondisable bit of A_HDCPCFG1 but you cannot enable the bypass. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    B0 = 0,
    #[doc = "1: You can still write to the bit bypencryption of A_HDCPCFG0 or encryptiondisable bit of A_HDCPCFG1 but you cannot enable the bypass. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    B1 = 1,
}
impl From<HdcpLock> for bool {
    #[inline(always)]
    fn from(variant: HdcpLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP_LOCK` reader - Lock the HDCP bypass and encryption disable mechanisms:"]
pub type HdcpLockR = crate::BitReader<HdcpLock>;
impl HdcpLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdcpLock {
        match self.bits {
            false => HdcpLock::B0,
            true => HdcpLock::B1,
        }
    }
    #[doc = "You can still write to the bit bypencryption of A_HDCPCFG0 or encryptiondisable bit of A_HDCPCFG1 but you cannot enable the bypass. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdcpLock::B0
    }
    #[doc = "You can still write to the bit bypencryption of A_HDCPCFG0 or encryptiondisable bit of A_HDCPCFG1 but you cannot enable the bypass. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdcpLock::B1
    }
}
#[doc = "Field `HDCP_LOCK` writer - Lock the HDCP bypass and encryption disable mechanisms:"]
pub type HdcpLockW<'a, REG> = crate::BitWriter<'a, REG, HdcpLock>;
impl<'a, REG> HdcpLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "You can still write to the bit bypencryption of A_HDCPCFG0 or encryptiondisable bit of A_HDCPCFG1 but you cannot enable the bypass. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdcpLock::B0)
    }
    #[doc = "You can still write to the bit bypencryption of A_HDCPCFG0 or encryptiondisable bit of A_HDCPCFG1 but you cannot enable the bypass. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdcpLock::B1)
    }
}
#[doc = "Field `SPARE` reader - Reserved as \"spare\" register with no associated functionality."]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - Reserved as \"spare\" register with no associated functionality."]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Software reset signal, active by writing a zero and auto cleared to 1 in the following cycle."]
    #[inline(always)]
    pub fn swreset(&self) -> SwresetR {
        SwresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable encryption without losing authentication"]
    #[inline(always)]
    pub fn encryptiondisable(&self) -> EncryptiondisableR {
        EncryptiondisableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the encoding of packet header in the tmdsch0 bit\\[0\\]
with cipher\\[2\\]
instead of the tmdsch0 bit\\[2\\]
Note: This bit must always be set to 1 for all PHYs."]
    #[inline(always)]
    pub fn ph2upshftenc(&self) -> Ph2upshftencR {
        Ph2upshftencR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disables the request to the API processor to verify the SHA1 message digest of a received KSV List"]
    #[inline(always)]
    pub fn dissha1check(&self) -> Dissha1checkR {
        Dissha1checkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock the HDCP bypass and encryption disable mechanisms:"]
    #[inline(always)]
    pub fn hdcp_lock(&self) -> HdcpLockR {
        HdcpLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved as \"spare\" register with no associated functionality."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset signal, active by writing a zero and auto cleared to 1 in the following cycle."]
    #[inline(always)]
    #[must_use]
    pub fn swreset(&mut self) -> SwresetW<AHdcpcfg1Spec> {
        SwresetW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable encryption without losing authentication"]
    #[inline(always)]
    #[must_use]
    pub fn encryptiondisable(&mut self) -> EncryptiondisableW<AHdcpcfg1Spec> {
        EncryptiondisableW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the encoding of packet header in the tmdsch0 bit\\[0\\]
with cipher\\[2\\]
instead of the tmdsch0 bit\\[2\\]
Note: This bit must always be set to 1 for all PHYs."]
    #[inline(always)]
    #[must_use]
    pub fn ph2upshftenc(&mut self) -> Ph2upshftencW<AHdcpcfg1Spec> {
        Ph2upshftencW::new(self, 2)
    }
    #[doc = "Bit 3 - Disables the request to the API processor to verify the SHA1 message digest of a received KSV List"]
    #[inline(always)]
    #[must_use]
    pub fn dissha1check(&mut self) -> Dissha1checkW<AHdcpcfg1Spec> {
        Dissha1checkW::new(self, 3)
    }
    #[doc = "Bit 4 - Lock the HDCP bypass and encryption disable mechanisms:"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_lock(&mut self) -> HdcpLockW<AHdcpcfg1Spec> {
        HdcpLockW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Reserved as \"spare\" register with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<AHdcpcfg1Spec> {
        SpareW::new(self, 5)
    }
}
#[doc = "Software reset signal, active by writing a zero and auto cleared to 1 in the following cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_hdcpcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHdcpcfg1Spec;
impl crate::RegisterSpec for AHdcpcfg1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_hdcpcfg1::R`](R) reader structure"]
impl crate::Readable for AHdcpcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`a_hdcpcfg1::W`](W) writer structure"]
impl crate::Writable for AHdcpcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets A_HDCPCFG1 to value 0x01"]
impl crate::Resettable for AHdcpcfg1Spec {
    const RESET_VALUE: u8 = 0x01;
}
