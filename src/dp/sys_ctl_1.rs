#[doc = "Register `SYS_CTL_1` reader"]
pub type R = crate::R<SysCtl1Spec>;
#[doc = "Register `SYS_CTL_1` writer"]
pub type W = crate::W<SysCtl1Spec>;
#[doc = "Video stream clock detect status control:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DetCtrl {
    #[doc = "1: Use force detect status"]
    B1 = 1,
    #[doc = "0: Use auto-detected status This bit's type is R/W."]
    B0 = 0,
}
impl From<DetCtrl> for bool {
    #[inline(always)]
    fn from(variant: DetCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DET_CTRL` reader - Video stream clock detect status control:"]
pub type DetCtrlR = crate::BitReader<DetCtrl>;
impl DetCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DetCtrl {
        match self.bits {
            true => DetCtrl::B1,
            false => DetCtrl::B0,
        }
    }
    #[doc = "Use force detect status"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DetCtrl::B1
    }
    #[doc = "Use auto-detected status This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DetCtrl::B0
    }
}
#[doc = "Field `DET_CTRL` writer - Video stream clock detect status control:"]
pub type DetCtrlW<'a, REG> = crate::BitWriter1C<'a, REG, DetCtrl>;
impl<'a, REG> DetCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use force detect status"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DetCtrl::B1)
    }
    #[doc = "Use auto-detected status This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DetCtrl::B0)
    }
}
#[doc = "Force video stream clock detect, this bit is \n\nonly active when DET_CTRL is 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceDet {
    #[doc = "1: Force video stream clock detected"]
    B1 = 1,
    #[doc = "0: Force video stream clock not detected This bit's type is R/W."]
    B0 = 0,
}
impl From<ForceDet> for bool {
    #[inline(always)]
    fn from(variant: ForceDet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_DET` reader - Force video stream clock detect, this bit is \n\nonly active when DET_CTRL is 1"]
pub type ForceDetR = crate::BitReader<ForceDet>;
impl ForceDetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceDet {
        match self.bits {
            true => ForceDet::B1,
            false => ForceDet::B0,
        }
    }
    #[doc = "Force video stream clock detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ForceDet::B1
    }
    #[doc = "Force video stream clock not detected This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ForceDet::B0
    }
}
#[doc = "Field `FORCE_DET` writer - Force video stream clock detect, this bit is \n\nonly active when DET_CTRL is 1"]
pub type ForceDetW<'a, REG> = crate::BitWriter1C<'a, REG, ForceDet>;
impl<'a, REG> ForceDetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force video stream clock detected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ForceDet::B1)
    }
    #[doc = "Force video stream clock not detected This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ForceDet::B0)
    }
}
#[doc = "Video stream clock detect status, It will not \n\naffect video output.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DetSta {
    #[doc = "1: Stream clock detected"]
    B1 = 1,
    #[doc = "0: Stream clock not detected Write any value to update the current status."]
    B0 = 0,
}
impl From<DetSta> for bool {
    #[inline(always)]
    fn from(variant: DetSta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DET_STA` reader - Video stream clock detect status, It will not \n\naffect video output."]
pub type DetStaR = crate::BitReader<DetSta>;
impl DetStaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DetSta {
        match self.bits {
            true => DetSta::B1,
            false => DetSta::B0,
        }
    }
    #[doc = "Stream clock detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DetSta::B1
    }
    #[doc = "Stream clock not detected Write any value to update the current status."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DetSta::B0
    }
}
#[doc = "Field `DET_STA` writer - Video stream clock detect status, It will not \n\naffect video output."]
pub type DetStaW<'a, REG> = crate::BitWriter<'a, REG, DetSta>;
impl<'a, REG> DetStaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stream clock detected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DetSta::B1)
    }
    #[doc = "Stream clock not detected Write any value to update the current status."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DetSta::B0)
    }
}
#[doc = "Field `HBR2_EYE_SY_CTRL` reader - HBR2 pattern control"]
pub type Hbr2EyeSyCtrlR = crate::FieldReader;
#[doc = "Field `HBR2_EYE_SY_CTRL` writer - HBR2 pattern control"]
pub type Hbr2EyeSyCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Video stream clock detect status control:"]
    #[inline(always)]
    pub fn det_ctrl(&self) -> DetCtrlR {
        DetCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force video stream clock detect, this bit is \n\nonly active when DET_CTRL is 1"]
    #[inline(always)]
    pub fn force_det(&self) -> ForceDetR {
        ForceDetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Video stream clock detect status, It will not \n\naffect video output."]
    #[inline(always)]
    pub fn det_sta(&self) -> DetStaR {
        DetStaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - HBR2 pattern control"]
    #[inline(always)]
    pub fn hbr2_eye_sy_ctrl(&self) -> Hbr2EyeSyCtrlR {
        Hbr2EyeSyCtrlR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Video stream clock detect status control:"]
    #[inline(always)]
    #[must_use]
    pub fn det_ctrl(&mut self) -> DetCtrlW<SysCtl1Spec> {
        DetCtrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Force video stream clock detect, this bit is \n\nonly active when DET_CTRL is 1"]
    #[inline(always)]
    #[must_use]
    pub fn force_det(&mut self) -> ForceDetW<SysCtl1Spec> {
        ForceDetW::new(self, 1)
    }
    #[doc = "Bit 2 - Video stream clock detect status, It will not \n\naffect video output."]
    #[inline(always)]
    #[must_use]
    pub fn det_sta(&mut self) -> DetStaW<SysCtl1Spec> {
        DetStaW::new(self, 2)
    }
    #[doc = "Bits 3:4 - HBR2 pattern control"]
    #[inline(always)]
    #[must_use]
    pub fn hbr2_eye_sy_ctrl(&mut self) -> Hbr2EyeSyCtrlW<SysCtl1Spec> {
        Hbr2EyeSyCtrlW::new(self, 3)
    }
}
#[doc = "System Control Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCtl1Spec;
impl crate::RegisterSpec for SysCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctl_1::R`](R) reader structure"]
impl crate::Readable for SysCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_ctl_1::W`](W) writer structure"]
impl crate::Writable for SysCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1b;
}
#[doc = "`reset()` method sets SYS_CTL_1 to value 0"]
impl crate::Resettable for SysCtl1Spec {
    const RESET_VALUE: u32 = 0;
}
