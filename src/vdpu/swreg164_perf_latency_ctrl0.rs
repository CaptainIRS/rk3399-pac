#[doc = "Register `SWREG164_PERF_LATENCY_CTRL0` reader"]
pub type R = crate::R<Swreg164PerfLatencyCtrl0Spec>;
#[doc = "Register `SWREG164_PERF_LATENCY_CTRL0` writer"]
pub type W = crate::W<Swreg164PerfLatencyCtrl0Spec>;
#[doc = "sw_axi_perf_work_e\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwAxiPerfWorkE {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwAxiPerfWorkE> for bool {
    #[inline(always)]
    fn from(variant: SwAxiPerfWorkE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_AXI_PERF_WORK_E` reader - sw_axi_perf_work_e"]
pub type SwAxiPerfWorkER = crate::BitReader<SwAxiPerfWorkE>;
impl SwAxiPerfWorkER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwAxiPerfWorkE {
        match self.bits {
            false => SwAxiPerfWorkE::B0,
            true => SwAxiPerfWorkE::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwAxiPerfWorkE::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwAxiPerfWorkE::B1
    }
}
#[doc = "Field `SW_AXI_PERF_WORK_E` writer - sw_axi_perf_work_e"]
pub type SwAxiPerfWorkEW<'a, REG> = crate::BitWriter<'a, REG, SwAxiPerfWorkE>;
impl<'a, REG> SwAxiPerfWorkEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwAxiPerfWorkE::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwAxiPerfWorkE::B1)
    }
}
#[doc = "sw_axi_perf_clr_e\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwAxiPerfClrE {
    #[doc = "0: software clear disable"]
    B0 = 0,
    #[doc = "1: software clear enalbe"]
    B1 = 1,
}
impl From<SwAxiPerfClrE> for bool {
    #[inline(always)]
    fn from(variant: SwAxiPerfClrE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_AXI_PERF_CLR_E` reader - sw_axi_perf_clr_e"]
pub type SwAxiPerfClrER = crate::BitReader<SwAxiPerfClrE>;
impl SwAxiPerfClrER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwAxiPerfClrE {
        match self.bits {
            false => SwAxiPerfClrE::B0,
            true => SwAxiPerfClrE::B1,
        }
    }
    #[doc = "software clear disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwAxiPerfClrE::B0
    }
    #[doc = "software clear enalbe"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwAxiPerfClrE::B1
    }
}
#[doc = "Field `SW_AXI_PERF_CLR_E` writer - sw_axi_perf_clr_e"]
pub type SwAxiPerfClrEW<'a, REG> = crate::BitWriter1C<'a, REG, SwAxiPerfClrE>;
impl<'a, REG> SwAxiPerfClrEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "software clear disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwAxiPerfClrE::B0)
    }
    #[doc = "software clear enalbe"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwAxiPerfClrE::B1)
    }
}
#[doc = "sw_axi_perf_frm_type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwAxiPerfFrmType {
    #[doc = "0: clear by frame end"]
    B0 = 0,
    #[doc = "1: clear by software configuration"]
    B1 = 1,
}
impl From<SwAxiPerfFrmType> for bool {
    #[inline(always)]
    fn from(variant: SwAxiPerfFrmType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_AXI_PERF_FRM_TYPE` reader - sw_axi_perf_frm_type"]
pub type SwAxiPerfFrmTypeR = crate::BitReader<SwAxiPerfFrmType>;
impl SwAxiPerfFrmTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwAxiPerfFrmType {
        match self.bits {
            false => SwAxiPerfFrmType::B0,
            true => SwAxiPerfFrmType::B1,
        }
    }
    #[doc = "clear by frame end"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwAxiPerfFrmType::B0
    }
    #[doc = "clear by software configuration"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwAxiPerfFrmType::B1
    }
}
#[doc = "Field `SW_AXI_PERF_FRM_TYPE` writer - sw_axi_perf_frm_type"]
pub type SwAxiPerfFrmTypeW<'a, REG> = crate::BitWriter<'a, REG, SwAxiPerfFrmType>;
impl<'a, REG> SwAxiPerfFrmTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear by frame end"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwAxiPerfFrmType::B0)
    }
    #[doc = "clear by software configuration"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwAxiPerfFrmType::B1)
    }
}
#[doc = "Field `SW_AXI_CNT_TYPE` reader - sw_axi_cnt_type\n\nsw_axi_cnt_type"]
pub type SwAxiCntTypeR = crate::BitReader;
#[doc = "Field `SW_AXI_CNT_TYPE` writer - sw_axi_cnt_type\n\nsw_axi_cnt_type"]
pub type SwAxiCntTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RD_LATENCY_ID` reader - sw_rd_latency_id"]
pub type SwRdLatencyIdR = crate::FieldReader;
#[doc = "Field `SW_RD_LATENCY_ID` writer - sw_rd_latency_id"]
pub type SwRdLatencyIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_RD_LATENCY_THR` reader - sw_rd_latency_thr"]
pub type SwRdLatencyThrR = crate::FieldReader<u16>;
#[doc = "Field `SW_RD_LATENCY_THR` writer - sw_rd_latency_thr"]
pub type SwRdLatencyThrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - sw_axi_perf_work_e"]
    #[inline(always)]
    pub fn sw_axi_perf_work_e(&self) -> SwAxiPerfWorkER {
        SwAxiPerfWorkER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sw_axi_perf_clr_e"]
    #[inline(always)]
    pub fn sw_axi_perf_clr_e(&self) -> SwAxiPerfClrER {
        SwAxiPerfClrER::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sw_axi_perf_frm_type"]
    #[inline(always)]
    pub fn sw_axi_perf_frm_type(&self) -> SwAxiPerfFrmTypeR {
        SwAxiPerfFrmTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_axi_cnt_type\n\nsw_axi_cnt_type"]
    #[inline(always)]
    pub fn sw_axi_cnt_type(&self) -> SwAxiCntTypeR {
        SwAxiCntTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - sw_rd_latency_id"]
    #[inline(always)]
    pub fn sw_rd_latency_id(&self) -> SwRdLatencyIdR {
        SwRdLatencyIdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:19 - sw_rd_latency_thr"]
    #[inline(always)]
    pub fn sw_rd_latency_thr(&self) -> SwRdLatencyThrR {
        SwRdLatencyThrR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - sw_axi_perf_work_e"]
    #[inline(always)]
    #[must_use]
    pub fn sw_axi_perf_work_e(&mut self) -> SwAxiPerfWorkEW<Swreg164PerfLatencyCtrl0Spec> {
        SwAxiPerfWorkEW::new(self, 0)
    }
    #[doc = "Bit 1 - sw_axi_perf_clr_e"]
    #[inline(always)]
    #[must_use]
    pub fn sw_axi_perf_clr_e(&mut self) -> SwAxiPerfClrEW<Swreg164PerfLatencyCtrl0Spec> {
        SwAxiPerfClrEW::new(self, 1)
    }
    #[doc = "Bit 2 - sw_axi_perf_frm_type"]
    #[inline(always)]
    #[must_use]
    pub fn sw_axi_perf_frm_type(&mut self) -> SwAxiPerfFrmTypeW<Swreg164PerfLatencyCtrl0Spec> {
        SwAxiPerfFrmTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_axi_cnt_type\n\nsw_axi_cnt_type"]
    #[inline(always)]
    #[must_use]
    pub fn sw_axi_cnt_type(&mut self) -> SwAxiCntTypeW<Swreg164PerfLatencyCtrl0Spec> {
        SwAxiCntTypeW::new(self, 3)
    }
    #[doc = "Bits 4:7 - sw_rd_latency_id"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rd_latency_id(&mut self) -> SwRdLatencyIdW<Swreg164PerfLatencyCtrl0Spec> {
        SwRdLatencyIdW::new(self, 4)
    }
    #[doc = "Bits 8:19 - sw_rd_latency_thr"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rd_latency_thr(&mut self) -> SwRdLatencyThrW<Swreg164PerfLatencyCtrl0Spec> {
        SwRdLatencyThrW::new(self, 8)
    }
}
#[doc = "Axi performance latency module contrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg164_perf_latency_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg164_perf_latency_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg164PerfLatencyCtrl0Spec;
impl crate::RegisterSpec for Swreg164PerfLatencyCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg164_perf_latency_ctrl0::R`](R) reader structure"]
impl crate::Readable for Swreg164PerfLatencyCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg164_perf_latency_ctrl0::W`](W) writer structure"]
impl crate::Writable for Swreg164PerfLatencyCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x02;
}
#[doc = "`reset()` method sets SWREG164_PERF_LATENCY_CTRL0 to value 0"]
impl crate::Resettable for Swreg164PerfLatencyCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
