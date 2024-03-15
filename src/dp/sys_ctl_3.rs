#[doc = "Register `SYS_CTL_3` reader"]
pub type R = crate::R<SysCtl3Spec>;
#[doc = "Register `SYS_CTL_3` writer"]
pub type W = crate::W<SysCtl3Spec>;
#[doc = "Stream valid control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidCtrl {
    #[doc = "1: Use video stream valid auto-detect This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Use video stream valid auto-detect This bit's type is R/W."]
    B0 = 0,
}
impl From<ValidCtrl> for bool {
    #[inline(always)]
    fn from(variant: ValidCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID_CTRL` reader - Stream valid control."]
pub type ValidCtrlR = crate::BitReader<ValidCtrl>;
impl ValidCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ValidCtrl {
        match self.bits {
            true => ValidCtrl::B1,
            false => ValidCtrl::B0,
        }
    }
    #[doc = "Use video stream valid auto-detect This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ValidCtrl::B1
    }
    #[doc = "Use video stream valid auto-detect This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ValidCtrl::B0
    }
}
#[doc = "Field `VALID_CTRL` writer - Stream valid control."]
pub type ValidCtrlW<'a, REG> = crate::BitWriter1C<'a, REG, ValidCtrl>;
impl<'a, REG> ValidCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use video stream valid auto-detect This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ValidCtrl::B1)
    }
    #[doc = "Use video stream valid auto-detect This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ValidCtrl::B0)
    }
}
#[doc = "Force stream valid, this bit only active when VALID_CTRL is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FValid {
    #[doc = "1: Force input video stream not valid. This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Force input video stream not valid. This bit's type is R/W."]
    B0 = 0,
}
impl From<FValid> for bool {
    #[inline(always)]
    fn from(variant: FValid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_VALID` reader - Force stream valid, this bit only active when VALID_CTRL is 1."]
pub type FValidR = crate::BitReader<FValid>;
impl FValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FValid {
        match self.bits {
            true => FValid::B1,
            false => FValid::B0,
        }
    }
    #[doc = "Force input video stream not valid. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FValid::B1
    }
    #[doc = "Force input video stream not valid. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FValid::B0
    }
}
#[doc = "Field `F_VALID` writer - Force stream valid, this bit only active when VALID_CTRL is 1."]
pub type FValidW<'a, REG> = crate::BitWriter<'a, REG, FValid>;
impl<'a, REG> FValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force input video stream not valid. This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FValid::B1)
    }
    #[doc = "Force input video stream not valid. This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FValid::B0)
    }
}
#[doc = "Input stream have constant video format, and this stream is valid to send out through link.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrmValid {
    #[doc = "1: Input stream is not valid. Write any value to update the current status. Hardware will not send out video through link when this bit is 0."]
    B1 = 1,
    #[doc = "0: Input stream is not valid. Write any value to update the current status. Hardware will not send out video through link when this bit is 0."]
    B0 = 0,
}
impl From<StrmValid> for bool {
    #[inline(always)]
    fn from(variant: StrmValid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRM_VALID` reader - Input stream have constant video format, and this stream is valid to send out through link."]
pub type StrmValidR = crate::BitReader<StrmValid>;
impl StrmValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StrmValid {
        match self.bits {
            true => StrmValid::B1,
            false => StrmValid::B0,
        }
    }
    #[doc = "Input stream is not valid. Write any value to update the current status. Hardware will not send out video through link when this bit is 0."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StrmValid::B1
    }
    #[doc = "Input stream is not valid. Write any value to update the current status. Hardware will not send out video through link when this bit is 0."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StrmValid::B0
    }
}
#[doc = "Field `STRM_VALID` writer - Input stream have constant video format, and this stream is valid to send out through link."]
pub type StrmValidW<'a, REG> = crate::BitWriter<'a, REG, StrmValid>;
impl<'a, REG> StrmValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input stream is not valid. Write any value to update the current status. Hardware will not send out video through link when this bit is 0."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StrmValid::B1)
    }
    #[doc = "Input stream is not valid. Write any value to update the current status. Hardware will not send out video through link when this bit is 0."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StrmValid::B0)
    }
}
#[doc = "HDCP ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdcpRdy {
    #[doc = "1: HDCP is not ready. This bit's type is RO. This bit is an indicator of whether HDCP is ready to perform. Usually, it is set as soon as HPD signal is detected as plugged."]
    B1 = 1,
    #[doc = "0: HDCP is not ready. This bit's type is RO. This bit is an indicator of whether HDCP is ready to perform. Usually, it is set as soon as HPD signal is detected as plugged."]
    B0 = 0,
}
impl From<HdcpRdy> for bool {
    #[inline(always)]
    fn from(variant: HdcpRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP_RDY` reader - HDCP ready status."]
pub type HdcpRdyR = crate::BitReader<HdcpRdy>;
impl HdcpRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdcpRdy {
        match self.bits {
            true => HdcpRdy::B1,
            false => HdcpRdy::B0,
        }
    }
    #[doc = "HDCP is not ready. This bit's type is RO. This bit is an indicator of whether HDCP is ready to perform. Usually, it is set as soon as HPD signal is detected as plugged."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdcpRdy::B1
    }
    #[doc = "HDCP is not ready. This bit's type is RO. This bit is an indicator of whether HDCP is ready to perform. Usually, it is set as soon as HPD signal is detected as plugged."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdcpRdy::B0
    }
}
#[doc = "Hot plug detect manual control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HpdCtrl {
    #[doc = "1: Use PIN_HPD state. This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Use PIN_HPD state. This bit's type is R/W."]
    B0 = 0,
}
impl From<HpdCtrl> for bool {
    #[inline(always)]
    fn from(variant: HpdCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPD_CTRL` reader - Hot plug detect manual control."]
pub type HpdCtrlR = crate::BitReader<HpdCtrl>;
impl HpdCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HpdCtrl {
        match self.bits {
            true => HpdCtrl::B1,
            false => HpdCtrl::B0,
        }
    }
    #[doc = "Use PIN_HPD state. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HpdCtrl::B1
    }
    #[doc = "Use PIN_HPD state. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HpdCtrl::B0
    }
}
#[doc = "Field `HPD_CTRL` writer - Hot plug detect manual control."]
pub type HpdCtrlW<'a, REG> = crate::BitWriter1C<'a, REG, HpdCtrl>;
impl<'a, REG> HpdCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use PIN_HPD state. This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HpdCtrl::B1)
    }
    #[doc = "Use PIN_HPD state. This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HpdCtrl::B0)
    }
}
#[doc = "Force hot plug detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FHpd {
    #[doc = "1: Force HPD 0. This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Force HPD 0. This bit's type is R/W."]
    B0 = 0,
}
impl From<FHpd> for bool {
    #[inline(always)]
    fn from(variant: FHpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_HPD` reader - Force hot plug detect."]
pub type FHpdR = crate::BitReader<FHpd>;
impl FHpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FHpd {
        match self.bits {
            true => FHpd::B1,
            false => FHpd::B0,
        }
    }
    #[doc = "Force HPD 0. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FHpd::B1
    }
    #[doc = "Force HPD 0. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FHpd::B0
    }
}
#[doc = "Field `F_HPD` writer - Force hot plug detect."]
pub type FHpdW<'a, REG> = crate::BitWriter<'a, REG, FHpd>;
impl<'a, REG> FHpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force HPD 0. This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FHpd::B1)
    }
    #[doc = "Force HPD 0. This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FHpd::B0)
    }
}
#[doc = "Hot plug detect status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HpdStatus {
    #[doc = "1: HPD is 0. This bit's type is RO. When this bit is 0, AUX CH will not work. Note that the HPD_STATUS is only changed after the change of the pin I_DP_HPD remains for no less than hot plug deglitch time. And the hot plug deglitch time is defined in HPD_DEGLITCH_L and HPD_DEGLITCH_H."]
    B1 = 1,
    #[doc = "0: HPD is 0. This bit's type is RO. When this bit is 0, AUX CH will not work. Note that the HPD_STATUS is only changed after the change of the pin I_DP_HPD remains for no less than hot plug deglitch time. And the hot plug deglitch time is defined in HPD_DEGLITCH_L and HPD_DEGLITCH_H."]
    B0 = 0,
}
impl From<HpdStatus> for bool {
    #[inline(always)]
    fn from(variant: HpdStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPD_STATUS` reader - Hot plug detect status."]
pub type HpdStatusR = crate::BitReader<HpdStatus>;
impl HpdStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HpdStatus {
        match self.bits {
            true => HpdStatus::B1,
            false => HpdStatus::B0,
        }
    }
    #[doc = "HPD is 0. This bit's type is RO. When this bit is 0, AUX CH will not work. Note that the HPD_STATUS is only changed after the change of the pin I_DP_HPD remains for no less than hot plug deglitch time. And the hot plug deglitch time is defined in HPD_DEGLITCH_L and HPD_DEGLITCH_H."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HpdStatus::B1
    }
    #[doc = "HPD is 0. This bit's type is RO. When this bit is 0, AUX CH will not work. Note that the HPD_STATUS is only changed after the change of the pin I_DP_HPD remains for no less than hot plug deglitch time. And the hot plug deglitch time is defined in HPD_DEGLITCH_L and HPD_DEGLITCH_H."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HpdStatus::B0
    }
}
impl R {
    #[doc = "Bit 0 - Stream valid control."]
    #[inline(always)]
    pub fn valid_ctrl(&self) -> ValidCtrlR {
        ValidCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force stream valid, this bit only active when VALID_CTRL is 1."]
    #[inline(always)]
    pub fn f_valid(&self) -> FValidR {
        FValidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input stream have constant video format, and this stream is valid to send out through link."]
    #[inline(always)]
    pub fn strm_valid(&self) -> StrmValidR {
        StrmValidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HDCP ready status."]
    #[inline(always)]
    pub fn hdcp_rdy(&self) -> HdcpRdyR {
        HdcpRdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hot plug detect manual control."]
    #[inline(always)]
    pub fn hpd_ctrl(&self) -> HpdCtrlR {
        HpdCtrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force hot plug detect."]
    #[inline(always)]
    pub fn f_hpd(&self) -> FHpdR {
        FHpdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hot plug detect status."]
    #[inline(always)]
    pub fn hpd_status(&self) -> HpdStatusR {
        HpdStatusR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stream valid control."]
    #[inline(always)]
    #[must_use]
    pub fn valid_ctrl(&mut self) -> ValidCtrlW<SysCtl3Spec> {
        ValidCtrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Force stream valid, this bit only active when VALID_CTRL is 1."]
    #[inline(always)]
    #[must_use]
    pub fn f_valid(&mut self) -> FValidW<SysCtl3Spec> {
        FValidW::new(self, 1)
    }
    #[doc = "Bit 2 - Input stream have constant video format, and this stream is valid to send out through link."]
    #[inline(always)]
    #[must_use]
    pub fn strm_valid(&mut self) -> StrmValidW<SysCtl3Spec> {
        StrmValidW::new(self, 2)
    }
    #[doc = "Bit 4 - Hot plug detect manual control."]
    #[inline(always)]
    #[must_use]
    pub fn hpd_ctrl(&mut self) -> HpdCtrlW<SysCtl3Spec> {
        HpdCtrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Force hot plug detect."]
    #[inline(always)]
    #[must_use]
    pub fn f_hpd(&mut self) -> FHpdW<SysCtl3Spec> {
        FHpdW::new(self, 5)
    }
}
#[doc = "System Control Register #3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctl_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctl_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCtl3Spec;
impl crate::RegisterSpec for SysCtl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctl_3::R`](R) reader structure"]
impl crate::Readable for SysCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_ctl_3::W`](W) writer structure"]
impl crate::Writable for SysCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x11;
}
#[doc = "`reset()` method sets SYS_CTL_3 to value 0"]
impl crate::Resettable for SysCtl3Spec {
    const RESET_VALUE: u32 = 0;
}
