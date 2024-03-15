#[doc = "Register `HDCP22REG_STAT` reader"]
pub type R = crate::R<Hdcp22regStatSpec>;
#[doc = "Register `HDCP22REG_STAT` writer"]
pub type W = crate::W<Hdcp22regStatSpec>;
#[doc = "Field `ST_HDCP2_CAPABLE` reader - HDCP 2.2 capable rise interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcp2CapableR = crate::BitReader;
#[doc = "Field `ST_HDCP2_CAPABLE` writer - HDCP 2.2 capable rise interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcp2CapableW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ST_HDCP2_NOT_CAPABLE` reader - HDCP 2.2 not capable rise interrupt status sticky bit. Clear by Write 1 to this bit field Bits Name Attr Description"]
pub type StHdcp2NotCapableR = crate::BitReader;
#[doc = "Field `ST_HDCP2_NOT_CAPABLE` writer - HDCP 2.2 not capable rise interrupt status sticky bit. Clear by Write 1 to this bit field Bits Name Attr Description"]
pub type StHdcp2NotCapableW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ST_HDCP_AUTHENTICATION_LOST` reader - HDCP 2.2 authentication lost interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcpAuthenticationLostR = crate::BitReader;
#[doc = "Field `ST_HDCP_AUTHENTICATION_LOST` writer - HDCP 2.2 authentication lost interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcpAuthenticationLostW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ST_HDCP_AUTHENTICATED` reader - HDCP 2.2 authenticated interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcpAuthenticatedR = crate::BitReader;
#[doc = "Field `ST_HDCP_AUTHENTICATED` writer - HDCP 2.2 authenticated interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcpAuthenticatedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ST_HDCP_AUTHENTICATION_FAIL` reader - HDCP 2.2 authentication fail interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcpAuthenticationFailR = crate::BitReader;
#[doc = "Field `ST_HDCP_AUTHENTICATION_FAIL` writer - HDCP 2.2 authentication fail interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcpAuthenticationFailW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ST_HDCP_DECRYPTED_CHG` reader - HDCP 2.2 decrypted value change interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcpDecryptedChgR = crate::BitReader;
#[doc = "Field `ST_HDCP_DECRYPTED_CHG` writer - HDCP 2.2 decrypted value change interrupt status sticky bit. Clear by Write 1 to this bit field"]
pub type StHdcpDecryptedChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - HDCP 2.2 capable rise interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    pub fn st_hdcp2_capable(&self) -> StHdcp2CapableR {
        StHdcp2CapableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HDCP 2.2 not capable rise interrupt status sticky bit. Clear by Write 1 to this bit field Bits Name Attr Description"]
    #[inline(always)]
    pub fn st_hdcp2_not_capable(&self) -> StHdcp2NotCapableR {
        StHdcp2NotCapableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HDCP 2.2 authentication lost interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    pub fn st_hdcp_authentication_lost(&self) -> StHdcpAuthenticationLostR {
        StHdcpAuthenticationLostR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HDCP 2.2 authenticated interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    pub fn st_hdcp_authenticated(&self) -> StHdcpAuthenticatedR {
        StHdcpAuthenticatedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HDCP 2.2 authentication fail interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    pub fn st_hdcp_authentication_fail(&self) -> StHdcpAuthenticationFailR {
        StHdcpAuthenticationFailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HDCP 2.2 decrypted value change interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    pub fn st_hdcp_decrypted_chg(&self) -> StHdcpDecryptedChgR {
        StHdcpDecryptedChgR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HDCP 2.2 capable rise interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    #[must_use]
    pub fn st_hdcp2_capable(&mut self) -> StHdcp2CapableW<Hdcp22regStatSpec> {
        StHdcp2CapableW::new(self, 0)
    }
    #[doc = "Bit 1 - HDCP 2.2 not capable rise interrupt status sticky bit. Clear by Write 1 to this bit field Bits Name Attr Description"]
    #[inline(always)]
    #[must_use]
    pub fn st_hdcp2_not_capable(&mut self) -> StHdcp2NotCapableW<Hdcp22regStatSpec> {
        StHdcp2NotCapableW::new(self, 1)
    }
    #[doc = "Bit 2 - HDCP 2.2 authentication lost interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    #[must_use]
    pub fn st_hdcp_authentication_lost(&mut self) -> StHdcpAuthenticationLostW<Hdcp22regStatSpec> {
        StHdcpAuthenticationLostW::new(self, 2)
    }
    #[doc = "Bit 3 - HDCP 2.2 authenticated interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    #[must_use]
    pub fn st_hdcp_authenticated(&mut self) -> StHdcpAuthenticatedW<Hdcp22regStatSpec> {
        StHdcpAuthenticatedW::new(self, 3)
    }
    #[doc = "Bit 4 - HDCP 2.2 authentication fail interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    #[must_use]
    pub fn st_hdcp_authentication_fail(&mut self) -> StHdcpAuthenticationFailW<Hdcp22regStatSpec> {
        StHdcpAuthenticationFailW::new(self, 4)
    }
    #[doc = "Bit 5 - HDCP 2.2 decrypted value change interrupt status sticky bit. Clear by Write 1 to this bit field"]
    #[inline(always)]
    #[must_use]
    pub fn st_hdcp_decrypted_chg(&mut self) -> StHdcpDecryptedChgW<Hdcp22regStatSpec> {
        StHdcpDecryptedChgW::new(self, 5)
    }
}
#[doc = "HDCP 2.2 capable rise interrupt status sticky bit. Clear by Write 1 to this bit field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hdcp22regStatSpec;
impl crate::RegisterSpec for Hdcp22regStatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp22reg_stat::R`](R) reader structure"]
impl crate::Readable for Hdcp22regStatSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcp22reg_stat::W`](W) writer structure"]
impl crate::Writable for Hdcp22regStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x3f;
}
#[doc = "`reset()` method sets HDCP22REG_STAT to value 0"]
impl crate::Resettable for Hdcp22regStatSpec {
    const RESET_VALUE: u8 = 0;
}
