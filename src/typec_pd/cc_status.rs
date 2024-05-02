#[doc = "Register `CC_STATUS` reader"]
pub type R = crate::R<CcStatusSpec>;
#[doc = "Field `CC1_State` reader - If (ROLE_CONTROL.CC1 = Rp)or \n\n(ConnectResult=0) \n\n00b: SRC.Open (Open,Rp) \n\n01b: SRC.Ra (below maximumvRa) \n\n10b: SRC.Rd (within the vRd range) \n\n11b:reserved \n\n \n\nIf \n\n(ROLE_CONTROL.CC1 \n\n= \n\nRd) \n\n(ConnectResult=1) \n\n00b: \n\nSNK.Open \n\n(Below \n\nmaximumvRa) \n\n01b: SNK.Default (Above minimum vRd-Connect) \n\n10b: \n\nSNK.Power1.5 \n\n(Above minimum \n\nvRd-\n\nConnect) DetectsRp-1.5A \n\n11b: \n\nSNK.Power3.0 \n\n(Above minimum \n\nvRd-\n\nConnect) DetectsRp-3.0A \n\n \n\nIf ROLE_CONTROL.CC1=Ra, this field is set to00b \n\nIf ROLE_CONTROL.CC1=Open, this field is set to00b \n\n \n\nThis \n\nfield \n\nalways \n\nreturns \n\n00b \n\nif \n\n(Looking4Connection=1) \n\nor \n\n(POWER_CONTROL.EnableVconn=1 \n\nand \n\nPOWER_CONTROL.PlugOrientation=0). \n\nOtherwise, the returned value depends upon \n\nROLE_CONTROL.CC1."]
pub type Cc1StateR = crate::FieldReader;
#[doc = "Field `CC2_State` reader - If \n\n(ROLE_CONTROL.CC2=Rp) \n\nor \n\n(ConnectResult=0) 00b: SRC.Open (Open,Rp) \n\n01b: SRC.Ra (below maximumvRa) \n\n10b: SRC.Rd (within the vRd range) \n\n11b:reserved \n\n \n\nIf \n\n(ROLE_CONTROL.CC2=Rd) \n\nor \n\n(ConnectResult=1) \n\n00b: \n\nSNK.Open \n\n(Below \n\nmaximumvRa) \n\n01b: SNK.Default (Above minimum vRd-Connect) \n\n10b: \n\nSNK.Power1.5 \n\n(Above minimum \n\nvRd-\n\nConnect) Detects Rp1.5A \n\n11b: \n\nSNK.Power3.0 \n\n(Above minimum \n\nvRd-\n\nConnect) Detects Rp3.0A \n\n \n\nIf ROLE_CONTROL.CC2=Ra, this field is set to00b \n\nIf ROLE_CONTROL.CC2=Open, this field is set to00b \n\n \n\nThis \n\nfield \n\nalways \n\nreturns \n\n00b \n\nif \n\n(Looking4Connection=1) \n\nor \n\n(POWER_CONTROL.EnableVconn=1 \n\nand \n\nPOWER_CONTROL.PlugOrientation=0). \n\nOtherwise, the returned value depends upon \n\nROLE_CONTROL.CC2."]
pub type Cc2StateR = crate::FieldReader;
#[doc = "Field `ConnectResult` reader - 0b: the TCPC is presenting Rp 1b: \n\nthe TCPC is presenting Rd"]
pub type ConnectResultR = crate::BitReader;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Looking4connection {
    #[doc = "0: TCPC is not actively looking for a connection. A transition from ’1’ to ’0’ indicates a potential connection has been found."]
    B0 = 0,
    #[doc = "1: TCPC is looking for a connection (toggling as a DRP or looking for a connection as Sink/Source only condition)"]
    B1 = 1,
}
impl From<Looking4connection> for bool {
    #[inline(always)]
    fn from(variant: Looking4connection) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Looking4Connection` reader - "]
pub type Looking4connectionR = crate::BitReader<Looking4connection>;
impl Looking4connectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Looking4connection {
        match self.bits {
            false => Looking4connection::B0,
            true => Looking4connection::B1,
        }
    }
    #[doc = "TCPC is not actively looking for a connection. A transition from ’1’ to ’0’ indicates a potential connection has been found."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Looking4connection::B0
    }
    #[doc = "TCPC is looking for a connection (toggling as a DRP or looking for a connection as Sink/Source only condition)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Looking4connection::B1
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue. Always reads0."]
pub type NotUsedR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - If (ROLE_CONTROL.CC1 = Rp)or \n\n(ConnectResult=0) \n\n00b: SRC.Open (Open,Rp) \n\n01b: SRC.Ra (below maximumvRa) \n\n10b: SRC.Rd (within the vRd range) \n\n11b:reserved \n\n \n\nIf \n\n(ROLE_CONTROL.CC1 \n\n= \n\nRd) \n\n(ConnectResult=1) \n\n00b: \n\nSNK.Open \n\n(Below \n\nmaximumvRa) \n\n01b: SNK.Default (Above minimum vRd-Connect) \n\n10b: \n\nSNK.Power1.5 \n\n(Above minimum \n\nvRd-\n\nConnect) DetectsRp-1.5A \n\n11b: \n\nSNK.Power3.0 \n\n(Above minimum \n\nvRd-\n\nConnect) DetectsRp-3.0A \n\n \n\nIf ROLE_CONTROL.CC1=Ra, this field is set to00b \n\nIf ROLE_CONTROL.CC1=Open, this field is set to00b \n\n \n\nThis \n\nfield \n\nalways \n\nreturns \n\n00b \n\nif \n\n(Looking4Connection=1) \n\nor \n\n(POWER_CONTROL.EnableVconn=1 \n\nand \n\nPOWER_CONTROL.PlugOrientation=0). \n\nOtherwise, the returned value depends upon \n\nROLE_CONTROL.CC1."]
    #[inline(always)]
    pub fn cc1_state(&self) -> Cc1StateR {
        Cc1StateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - If \n\n(ROLE_CONTROL.CC2=Rp) \n\nor \n\n(ConnectResult=0) 00b: SRC.Open (Open,Rp) \n\n01b: SRC.Ra (below maximumvRa) \n\n10b: SRC.Rd (within the vRd range) \n\n11b:reserved \n\n \n\nIf \n\n(ROLE_CONTROL.CC2=Rd) \n\nor \n\n(ConnectResult=1) \n\n00b: \n\nSNK.Open \n\n(Below \n\nmaximumvRa) \n\n01b: SNK.Default (Above minimum vRd-Connect) \n\n10b: \n\nSNK.Power1.5 \n\n(Above minimum \n\nvRd-\n\nConnect) Detects Rp1.5A \n\n11b: \n\nSNK.Power3.0 \n\n(Above minimum \n\nvRd-\n\nConnect) Detects Rp3.0A \n\n \n\nIf ROLE_CONTROL.CC2=Ra, this field is set to00b \n\nIf ROLE_CONTROL.CC2=Open, this field is set to00b \n\n \n\nThis \n\nfield \n\nalways \n\nreturns \n\n00b \n\nif \n\n(Looking4Connection=1) \n\nor \n\n(POWER_CONTROL.EnableVconn=1 \n\nand \n\nPOWER_CONTROL.PlugOrientation=0). \n\nOtherwise, the returned value depends upon \n\nROLE_CONTROL.CC2."]
    #[inline(always)]
    pub fn cc2_state(&self) -> Cc2StateR {
        Cc2StateR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 0b: the TCPC is presenting Rp 1b: \n\nthe TCPC is presenting Rd"]
    #[inline(always)]
    pub fn connect_result(&self) -> ConnectResultR {
        ConnectResultR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn looking4connection(&self) -> Looking4connectionR {
        Looking4connectionR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "CC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcStatusSpec;
impl crate::RegisterSpec for CcStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_status::R`](R) reader structure"]
impl crate::Readable for CcStatusSpec {}
#[doc = "`reset()` method sets CC_STATUS to value 0"]
impl crate::Resettable for CcStatusSpec {
    const RESET_VALUE: u32 = 0;
}
