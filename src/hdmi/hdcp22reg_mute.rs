#[doc = "Register `HDCP22REG_MUTE` reader"]
pub type R = crate::R<Hdcp22regMuteSpec>;
#[doc = "Register `HDCP22REG_MUTE` writer"]
pub type W = crate::W<Hdcp22regMuteSpec>;
#[doc = "Field `MUTE_HDCP2_CAPABLE` reader - Active high interrupt mute to HDCP 2.2 capable rise interrupt status"]
pub type MuteHdcp2CapableR = crate::BitReader;
#[doc = "Field `MUTE_HDCP2_CAPABLE` writer - Active high interrupt mute to HDCP 2.2 capable rise interrupt status"]
pub type MuteHdcp2CapableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTE_HDCP2_NOT_CAPABLE` reader - Active high interrupt mute to HDCP 2.2 not capable rise interrupt status"]
pub type MuteHdcp2NotCapableR = crate::BitReader;
#[doc = "Field `MUTE_HDCP2_NOT_CAPABLE` writer - Active high interrupt mute to HDCP 2.2 not capable rise interrupt status"]
pub type MuteHdcp2NotCapableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTE_HDCP_AUTHENTICATION_LOST` reader - Active high interrupt mute to HDCP 2.2 authentication lost interrupt status"]
pub type MuteHdcpAuthenticationLostR = crate::BitReader;
#[doc = "Field `MUTE_HDCP_AUTHENTICATION_LOST` writer - Active high interrupt mute to HDCP 2.2 authentication lost interrupt status"]
pub type MuteHdcpAuthenticationLostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTE_HDCP_AUTHENTICATED` reader - Active high interrupt mute to HDCP 2.2 authenticated interrupt status"]
pub type MuteHdcpAuthenticatedR = crate::BitReader;
#[doc = "Field `MUTE_HDCP_AUTHENTICATED` writer - Active high interrupt mute to HDCP 2.2 authenticated interrupt status"]
pub type MuteHdcpAuthenticatedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTE_HDCP_AUTHENTICATION_FAIL` reader - Active high interrupt mute to HDCP 2.2 authentication fail interrupt status"]
pub type MuteHdcpAuthenticationFailR = crate::BitReader;
#[doc = "Field `MUTE_HDCP_AUTHENTICATION_FAIL` writer - Active high interrupt mute to HDCP 2.2 authentication fail interrupt status"]
pub type MuteHdcpAuthenticationFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTE_HDCP_DECRYPTED_CHG` reader - Active high interrupt mute to HDCP 2.2 decrypted value change interrupt status"]
pub type MuteHdcpDecryptedChgR = crate::BitReader;
#[doc = "Field `MUTE_HDCP_DECRYPTED_CHG` writer - Active high interrupt mute to HDCP 2.2 decrypted value change interrupt status"]
pub type MuteHdcpDecryptedChgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Active high interrupt mute to HDCP 2.2 capable rise interrupt status"]
    #[inline(always)]
    pub fn mute_hdcp2_capable(&self) -> MuteHdcp2CapableR {
        MuteHdcp2CapableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active high interrupt mute to HDCP 2.2 not capable rise interrupt status"]
    #[inline(always)]
    pub fn mute_hdcp2_not_capable(&self) -> MuteHdcp2NotCapableR {
        MuteHdcp2NotCapableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active high interrupt mute to HDCP 2.2 authentication lost interrupt status"]
    #[inline(always)]
    pub fn mute_hdcp_authentication_lost(&self) -> MuteHdcpAuthenticationLostR {
        MuteHdcpAuthenticationLostR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active high interrupt mute to HDCP 2.2 authenticated interrupt status"]
    #[inline(always)]
    pub fn mute_hdcp_authenticated(&self) -> MuteHdcpAuthenticatedR {
        MuteHdcpAuthenticatedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active high interrupt mute to HDCP 2.2 authentication fail interrupt status"]
    #[inline(always)]
    pub fn mute_hdcp_authentication_fail(&self) -> MuteHdcpAuthenticationFailR {
        MuteHdcpAuthenticationFailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active high interrupt mute to HDCP 2.2 decrypted value change interrupt status"]
    #[inline(always)]
    pub fn mute_hdcp_decrypted_chg(&self) -> MuteHdcpDecryptedChgR {
        MuteHdcpDecryptedChgR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high interrupt mute to HDCP 2.2 capable rise interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mute_hdcp2_capable(&mut self) -> MuteHdcp2CapableW<Hdcp22regMuteSpec> {
        MuteHdcp2CapableW::new(self, 0)
    }
    #[doc = "Bit 1 - Active high interrupt mute to HDCP 2.2 not capable rise interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mute_hdcp2_not_capable(&mut self) -> MuteHdcp2NotCapableW<Hdcp22regMuteSpec> {
        MuteHdcp2NotCapableW::new(self, 1)
    }
    #[doc = "Bit 2 - Active high interrupt mute to HDCP 2.2 authentication lost interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mute_hdcp_authentication_lost(
        &mut self,
    ) -> MuteHdcpAuthenticationLostW<Hdcp22regMuteSpec> {
        MuteHdcpAuthenticationLostW::new(self, 2)
    }
    #[doc = "Bit 3 - Active high interrupt mute to HDCP 2.2 authenticated interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mute_hdcp_authenticated(&mut self) -> MuteHdcpAuthenticatedW<Hdcp22regMuteSpec> {
        MuteHdcpAuthenticatedW::new(self, 3)
    }
    #[doc = "Bit 4 - Active high interrupt mute to HDCP 2.2 authentication fail interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mute_hdcp_authentication_fail(
        &mut self,
    ) -> MuteHdcpAuthenticationFailW<Hdcp22regMuteSpec> {
        MuteHdcpAuthenticationFailW::new(self, 4)
    }
    #[doc = "Bit 5 - Active high interrupt mute to HDCP 2.2 decrypted value change interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mute_hdcp_decrypted_chg(&mut self) -> MuteHdcpDecryptedChgW<Hdcp22regMuteSpec> {
        MuteHdcpDecryptedChgW::new(self, 5)
    }
}
#[doc = "Active high interrupt mute to HDCP 2.2 capable rise interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_mute::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_mute::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hdcp22regMuteSpec;
impl crate::RegisterSpec for Hdcp22regMuteSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp22reg_mute::R`](R) reader structure"]
impl crate::Readable for Hdcp22regMuteSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcp22reg_mute::W`](W) writer structure"]
impl crate::Writable for Hdcp22regMuteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP22REG_MUTE to value 0x3f"]
impl crate::Resettable for Hdcp22regMuteSpec {
    const RESET_VALUE: u8 = 0x3f;
}
