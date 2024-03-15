#[doc = "Register `HDCP22REG_MASK` reader"]
pub type R = crate::R<Hdcp22regMaskSpec>;
#[doc = "Register `HDCP22REG_MASK` writer"]
pub type W = crate::W<Hdcp22regMaskSpec>;
#[doc = "Field `MASK_HDCP2_CAPABLE` reader - Active high interrupt mask to HDCP 2.2 capable rise interrupt status"]
pub type MaskHdcp2CapableR = crate::BitReader;
#[doc = "Field `MASK_HDCP2_CAPABLE` writer - Active high interrupt mask to HDCP 2.2 capable rise interrupt status"]
pub type MaskHdcp2CapableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_HDCP2_NOT_CAPABLE` reader - Active high interrupt mask to HDCP 2.2 not capable rise interrupt status"]
pub type MaskHdcp2NotCapableR = crate::BitReader;
#[doc = "Field `MASK_HDCP2_NOT_CAPABLE` writer - Active high interrupt mask to HDCP 2.2 not capable rise interrupt status"]
pub type MaskHdcp2NotCapableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_HDCP_AUTHENTICATION_LOST` reader - Active high interrupt mask to HDCP 2.2 authentication lost interrupt status"]
pub type MaskHdcpAuthenticationLostR = crate::BitReader;
#[doc = "Field `MASK_HDCP_AUTHENTICATION_LOST` writer - Active high interrupt mask to HDCP 2.2 authentication lost interrupt status"]
pub type MaskHdcpAuthenticationLostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_HDCP_AUTHENTICATED` reader - Active high interrupt mask to HDCP 2.2 authenticated interrupt status"]
pub type MaskHdcpAuthenticatedR = crate::BitReader;
#[doc = "Field `MASK_HDCP_AUTHENTICATED` writer - Active high interrupt mask to HDCP 2.2 authenticated interrupt status"]
pub type MaskHdcpAuthenticatedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_HDCP_AUTHENTICATION_FAIL` reader - Active high interrupt mask to HDCP 2.2 authentication fail interrupt status"]
pub type MaskHdcpAuthenticationFailR = crate::BitReader;
#[doc = "Field `MASK_HDCP_AUTHENTICATION_FAIL` writer - Active high interrupt mask to HDCP 2.2 authentication fail interrupt status"]
pub type MaskHdcpAuthenticationFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_HDCP_DECRYPTED_CHG` reader - Active high interrupt mask to HDCP 2.2 decrypted value change interrupt status"]
pub type MaskHdcpDecryptedChgR = crate::BitReader;
#[doc = "Field `MASK_HDCP_DECRYPTED_CHG` writer - Active high interrupt mask to HDCP 2.2 decrypted value change interrupt status"]
pub type MaskHdcpDecryptedChgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Active high interrupt mask to HDCP 2.2 capable rise interrupt status"]
    #[inline(always)]
    pub fn mask_hdcp2_capable(&self) -> MaskHdcp2CapableR {
        MaskHdcp2CapableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active high interrupt mask to HDCP 2.2 not capable rise interrupt status"]
    #[inline(always)]
    pub fn mask_hdcp2_not_capable(&self) -> MaskHdcp2NotCapableR {
        MaskHdcp2NotCapableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active high interrupt mask to HDCP 2.2 authentication lost interrupt status"]
    #[inline(always)]
    pub fn mask_hdcp_authentication_lost(&self) -> MaskHdcpAuthenticationLostR {
        MaskHdcpAuthenticationLostR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active high interrupt mask to HDCP 2.2 authenticated interrupt status"]
    #[inline(always)]
    pub fn mask_hdcp_authenticated(&self) -> MaskHdcpAuthenticatedR {
        MaskHdcpAuthenticatedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active high interrupt mask to HDCP 2.2 authentication fail interrupt status"]
    #[inline(always)]
    pub fn mask_hdcp_authentication_fail(&self) -> MaskHdcpAuthenticationFailR {
        MaskHdcpAuthenticationFailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active high interrupt mask to HDCP 2.2 decrypted value change interrupt status"]
    #[inline(always)]
    pub fn mask_hdcp_decrypted_chg(&self) -> MaskHdcpDecryptedChgR {
        MaskHdcpDecryptedChgR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high interrupt mask to HDCP 2.2 capable rise interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hdcp2_capable(&mut self) -> MaskHdcp2CapableW<Hdcp22regMaskSpec> {
        MaskHdcp2CapableW::new(self, 0)
    }
    #[doc = "Bit 1 - Active high interrupt mask to HDCP 2.2 not capable rise interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hdcp2_not_capable(&mut self) -> MaskHdcp2NotCapableW<Hdcp22regMaskSpec> {
        MaskHdcp2NotCapableW::new(self, 1)
    }
    #[doc = "Bit 2 - Active high interrupt mask to HDCP 2.2 authentication lost interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hdcp_authentication_lost(
        &mut self,
    ) -> MaskHdcpAuthenticationLostW<Hdcp22regMaskSpec> {
        MaskHdcpAuthenticationLostW::new(self, 2)
    }
    #[doc = "Bit 3 - Active high interrupt mask to HDCP 2.2 authenticated interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hdcp_authenticated(&mut self) -> MaskHdcpAuthenticatedW<Hdcp22regMaskSpec> {
        MaskHdcpAuthenticatedW::new(self, 3)
    }
    #[doc = "Bit 4 - Active high interrupt mask to HDCP 2.2 authentication fail interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hdcp_authentication_fail(
        &mut self,
    ) -> MaskHdcpAuthenticationFailW<Hdcp22regMaskSpec> {
        MaskHdcpAuthenticationFailW::new(self, 4)
    }
    #[doc = "Bit 5 - Active high interrupt mask to HDCP 2.2 decrypted value change interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hdcp_decrypted_chg(&mut self) -> MaskHdcpDecryptedChgW<Hdcp22regMaskSpec> {
        MaskHdcpDecryptedChgW::new(self, 5)
    }
}
#[doc = "Active high interrupt mask to HDCP 2.2 capable rise interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hdcp22regMaskSpec;
impl crate::RegisterSpec for Hdcp22regMaskSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp22reg_mask::R`](R) reader structure"]
impl crate::Readable for Hdcp22regMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcp22reg_mask::W`](W) writer structure"]
impl crate::Writable for Hdcp22regMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP22REG_MASK to value 0x3f"]
impl crate::Resettable for Hdcp22regMaskSpec {
    const RESET_VALUE: u8 = 0x3f;
}
