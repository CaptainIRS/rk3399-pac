#[doc = "Register `HDCP22REG_STS` reader"]
pub type R = crate::R<Hdcp22regStsSpec>;
#[doc = "HDCP 2.2 HPD external interface status after lock mechanism (hdcp22reg_ctrl.hdcp22_switch_lock, hdcp22reg_ctrl.hdcp22_ovr_en and hdcp22reg_ctrl.hdcp22_ovr_val).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdmiHpdSts {
    #[doc = "0: Sink detected (HPD line set)"]
    B0 = 0,
    #[doc = "1: Sink detected (HPD line set)"]
    B1 = 1,
}
impl From<HdmiHpdSts> for bool {
    #[inline(always)]
    fn from(variant: HdmiHpdSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_HPD_STS` reader - HDCP 2.2 HPD external interface status after lock mechanism (hdcp22reg_ctrl.hdcp22_switch_lock, hdcp22reg_ctrl.hdcp22_ovr_en and hdcp22reg_ctrl.hdcp22_ovr_val)."]
pub type HdmiHpdStsR = crate::BitReader<HdmiHpdSts>;
impl HdmiHpdStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdmiHpdSts {
        match self.bits {
            false => HdmiHpdSts::B0,
            true => HdmiHpdSts::B1,
        }
    }
    #[doc = "Sink detected (HPD line set)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdmiHpdSts::B0
    }
    #[doc = "Sink detected (HPD line set)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdmiHpdSts::B1
    }
}
#[doc = "HDCP 2.2 AVMUTE external interface status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdcpAvmuteSts {
    #[doc = "0: External HDCP AVMUTE is set (audio/video should be muted)"]
    B0 = 0,
    #[doc = "1: External HDCP AVMUTE is set (audio/video should be muted)"]
    B1 = 1,
}
impl From<HdcpAvmuteSts> for bool {
    #[inline(always)]
    fn from(variant: HdcpAvmuteSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP_AVMUTE_STS` reader - HDCP 2.2 AVMUTE external interface status."]
pub type HdcpAvmuteStsR = crate::BitReader<HdcpAvmuteSts>;
impl HdcpAvmuteStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdcpAvmuteSts {
        match self.bits {
            false => HdcpAvmuteSts::B0,
            true => HdcpAvmuteSts::B1,
        }
    }
    #[doc = "External HDCP AVMUTE is set (audio/video should be muted)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdcpAvmuteSts::B0
    }
    #[doc = "External HDCP AVMUTE is set (audio/video should be muted)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdcpAvmuteSts::B1
    }
}
#[doc = "HDCP 2.2 HDCP 2.2 versus 1.4 switch status after lock mechanism (hdcp22reg_ctrl.hdcp22_switch_lock, hdcp22reg_ctrl.hdcp22_ovr_en and hdcp22reg_ctrl.hdcp22_ovr_val).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22SwitchSts {
    #[doc = "0: HDCP 2.2 selected"]
    B0 = 0,
    #[doc = "1: HDCP 2.2 selected"]
    B1 = 1,
}
impl From<Hdcp22SwitchSts> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22SwitchSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_SWITCH_STS` reader - HDCP 2.2 HDCP 2.2 versus 1.4 switch status after lock mechanism (hdcp22reg_ctrl.hdcp22_switch_lock, hdcp22reg_ctrl.hdcp22_ovr_en and hdcp22reg_ctrl.hdcp22_ovr_val)."]
pub type Hdcp22SwitchStsR = crate::BitReader<Hdcp22SwitchSts>;
impl Hdcp22SwitchStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22SwitchSts {
        match self.bits {
            false => Hdcp22SwitchSts::B0,
            true => Hdcp22SwitchSts::B1,
        }
    }
    #[doc = "HDCP 2.2 selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22SwitchSts::B0
    }
    #[doc = "HDCP 2.2 selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22SwitchSts::B1
    }
}
#[doc = "Field `HDCP_DECRYPTED_STS` reader - Value of HDCP 2.2 ist_hdcp_decrypted line. Provided for debug only"]
pub type HdcpDecryptedStsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HDCP 2.2 HPD external interface status after lock mechanism (hdcp22reg_ctrl.hdcp22_switch_lock, hdcp22reg_ctrl.hdcp22_ovr_en and hdcp22reg_ctrl.hdcp22_ovr_val)."]
    #[inline(always)]
    pub fn hdmi_hpd_sts(&self) -> HdmiHpdStsR {
        HdmiHpdStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HDCP 2.2 AVMUTE external interface status."]
    #[inline(always)]
    pub fn hdcp_avmute_sts(&self) -> HdcpAvmuteStsR {
        HdcpAvmuteStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HDCP 2.2 HDCP 2.2 versus 1.4 switch status after lock mechanism (hdcp22reg_ctrl.hdcp22_switch_lock, hdcp22reg_ctrl.hdcp22_ovr_en and hdcp22reg_ctrl.hdcp22_ovr_val)."]
    #[inline(always)]
    pub fn hdcp22_switch_sts(&self) -> Hdcp22SwitchStsR {
        Hdcp22SwitchStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Value of HDCP 2.2 ist_hdcp_decrypted line. Provided for debug only"]
    #[inline(always)]
    pub fn hdcp_decrypted_sts(&self) -> HdcpDecryptedStsR {
        HdcpDecryptedStsR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "HDCP 2.2 HPD external interface status after lock mechanism (hdcp22reg_ctrl.hdcp22_switch_lock, hdcp22reg_ctrl.hdcp22_ovr_en and hdcp22reg_ctrl.hdcp22_ovr_val). 1'b0: Sink not detected (HPD line clear ) 1'b1: Sink detected (HPD line set)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hdcp22regStsSpec;
impl crate::RegisterSpec for Hdcp22regStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp22reg_sts::R`](R) reader structure"]
impl crate::Readable for Hdcp22regStsSpec {}
#[doc = "`reset()` method sets HDCP22REG_STS to value 0"]
impl crate::Resettable for Hdcp22regStsSpec {
    const RESET_VALUE: u8 = 0;
}
