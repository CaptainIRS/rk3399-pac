#[doc = "Register `SOFTRST_CON17` reader"]
pub type R = crate::R<SoftrstCon17Spec>;
#[doc = "Register `SOFTRST_CON17` writer"]
pub type W = crate::W<SoftrstCon17Spec>;
#[doc = "Field `ARESETN_VOP0_NOC_REQ` reader - aresetn_vop0_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVop0NocReqR = crate::BitReader;
#[doc = "Field `ARESETN_VOP0_NOC_REQ` writer - aresetn_vop0_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVop0NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_VOP1_NOC_REQ` reader - aresetn_vop1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVop1NocReqR = crate::BitReader;
#[doc = "Field `ARESETN_VOP1_NOC_REQ` writer - aresetn_vop1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVop1NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_VOP0_REQ` reader - aresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVop0ReqR = crate::BitReader;
#[doc = "Field `ARESETN_VOP0_REQ` writer - aresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVop0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_VOP1_REQ` reader - aresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVop1ReqR = crate::BitReader;
#[doc = "Field `ARESETN_VOP1_REQ` writer - aresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVop1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_VOP0_NOC_REQ` reader - hresetn_vop0_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVop0NocReqR = crate::BitReader;
#[doc = "Field `HRESETN_VOP0_NOC_REQ` writer - hresetn_vop0_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVop0NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_VOP1_NOC_REQ` reader - hresetn_vop1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVop1NocReqR = crate::BitReader;
#[doc = "Field `HRESETN_VOP1_NOC_REQ` writer - hresetn_vop1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVop1NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_VOP0_REQ` reader - hresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVop0ReqR = crate::BitReader;
#[doc = "Field `HRESETN_VOP0_REQ` writer - hresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVop0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_VOP1_REQ` reader - hresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVop1ReqR = crate::BitReader;
#[doc = "Field `HRESETN_VOP1_REQ` writer - hresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVop1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRESETN_VOP0_REQ` reader - dresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
pub type DresetnVop0ReqR = crate::BitReader;
#[doc = "Field `DRESETN_VOP0_REQ` writer - dresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
pub type DresetnVop0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRESETN_VOP1_REQ` reader - dresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
pub type DresetnVop1ReqR = crate::BitReader;
#[doc = "Field `DRESETN_VOP1_REQ` writer - dresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
pub type DresetnVop1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_VOP0_PWM_REQ` reader - resetn_vop0_pwm request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnVop0PwmReqR = crate::BitReader;
#[doc = "Field `RESETN_VOP0_PWM_REQ` writer - resetn_vop0_pwm request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnVop0PwmReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_VOP1_PWM_REQ` reader - resetn_vop1_pwm request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnVop1PwmReqR = crate::BitReader;
#[doc = "Field `RESETN_VOP1_PWM_REQ` writer - resetn_vop1_pwm request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnVop1PwmReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_EDP_NOC_REQ` reader - presetn_edp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnEdpNocReqR = crate::BitReader;
#[doc = "Field `PRESETN_EDP_NOC_REQ` writer - presetn_edp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnEdpNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_EDP_CTRL_REQ` reader - presetn_edp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnEdpCtrlReqR = crate::BitReader;
#[doc = "Field `PRESETN_EDP_CTRL_REQ` writer - presetn_edp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnEdpCtrlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aresetn_vop0_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vop0_noc_req(&self) -> AresetnVop0NocReqR {
        AresetnVop0NocReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aresetn_vop1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vop1_noc_req(&self) -> AresetnVop1NocReqR {
        AresetnVop1NocReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vop0_req(&self) -> AresetnVop0ReqR {
        AresetnVop0ReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vop1_req(&self) -> AresetnVop1ReqR {
        AresetnVop1ReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hresetn_vop0_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_vop0_noc_req(&self) -> HresetnVop0NocReqR {
        HresetnVop0NocReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hresetn_vop1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_vop1_noc_req(&self) -> HresetnVop1NocReqR {
        HresetnVop1NocReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - hresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_vop0_req(&self) -> HresetnVop0ReqR {
        HresetnVop0ReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - hresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_vop1_req(&self) -> HresetnVop1ReqR {
        HresetnVop1ReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - dresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn dresetn_vop0_req(&self) -> DresetnVop0ReqR {
        DresetnVop0ReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - dresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn dresetn_vop1_req(&self) -> DresetnVop1ReqR {
        DresetnVop1ReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - resetn_vop0_pwm request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_vop0_pwm_req(&self) -> ResetnVop0PwmReqR {
        ResetnVop0PwmReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - resetn_vop1_pwm request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_vop1_pwm_req(&self) -> ResetnVop1PwmReqR {
        ResetnVop1PwmReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - presetn_edp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_edp_noc_req(&self) -> PresetnEdpNocReqR {
        PresetnEdpNocReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - presetn_edp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_edp_ctrl_req(&self) -> PresetnEdpCtrlReqR {
        PresetnEdpCtrlReqR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aresetn_vop0_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vop0_noc_req(&mut self) -> AresetnVop0NocReqW<SoftrstCon17Spec> {
        AresetnVop0NocReqW::new(self, 0)
    }
    #[doc = "Bit 1 - aresetn_vop1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vop1_noc_req(&mut self) -> AresetnVop1NocReqW<SoftrstCon17Spec> {
        AresetnVop1NocReqW::new(self, 1)
    }
    #[doc = "Bit 2 - aresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vop0_req(&mut self) -> AresetnVop0ReqW<SoftrstCon17Spec> {
        AresetnVop0ReqW::new(self, 2)
    }
    #[doc = "Bit 3 - aresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vop1_req(&mut self) -> AresetnVop1ReqW<SoftrstCon17Spec> {
        AresetnVop1ReqW::new(self, 3)
    }
    #[doc = "Bit 4 - hresetn_vop0_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_vop0_noc_req(&mut self) -> HresetnVop0NocReqW<SoftrstCon17Spec> {
        HresetnVop0NocReqW::new(self, 4)
    }
    #[doc = "Bit 5 - hresetn_vop1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_vop1_noc_req(&mut self) -> HresetnVop1NocReqW<SoftrstCon17Spec> {
        HresetnVop1NocReqW::new(self, 5)
    }
    #[doc = "Bit 6 - hresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_vop0_req(&mut self) -> HresetnVop0ReqW<SoftrstCon17Spec> {
        HresetnVop0ReqW::new(self, 6)
    }
    #[doc = "Bit 7 - hresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_vop1_req(&mut self) -> HresetnVop1ReqW<SoftrstCon17Spec> {
        HresetnVop1ReqW::new(self, 7)
    }
    #[doc = "Bit 8 - dresetn_vop0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn dresetn_vop0_req(&mut self) -> DresetnVop0ReqW<SoftrstCon17Spec> {
        DresetnVop0ReqW::new(self, 8)
    }
    #[doc = "Bit 9 - dresetn_vop1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn dresetn_vop1_req(&mut self) -> DresetnVop1ReqW<SoftrstCon17Spec> {
        DresetnVop1ReqW::new(self, 9)
    }
    #[doc = "Bit 10 - resetn_vop0_pwm request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_vop0_pwm_req(&mut self) -> ResetnVop0PwmReqW<SoftrstCon17Spec> {
        ResetnVop0PwmReqW::new(self, 10)
    }
    #[doc = "Bit 11 - resetn_vop1_pwm request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_vop1_pwm_req(&mut self) -> ResetnVop1PwmReqW<SoftrstCon17Spec> {
        ResetnVop1PwmReqW::new(self, 11)
    }
    #[doc = "Bit 12 - presetn_edp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_edp_noc_req(&mut self) -> PresetnEdpNocReqW<SoftrstCon17Spec> {
        PresetnEdpNocReqW::new(self, 12)
    }
    #[doc = "Bit 13 - presetn_edp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_edp_ctrl_req(&mut self) -> PresetnEdpCtrlReqW<SoftrstCon17Spec> {
        PresetnEdpCtrlReqW::new(self, 13)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon17Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon17Spec;
impl crate::RegisterSpec for SoftrstCon17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con17::R`](R) reader structure"]
impl crate::Readable for SoftrstCon17Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con17::W`](W) writer structure"]
impl crate::Writable for SoftrstCon17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON17 to value 0"]
impl crate::Resettable for SoftrstCon17Spec {
    const RESET_VALUE: u32 = 0;
}
