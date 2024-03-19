#[doc = "Register `CRU_SOFTRST_CON20` reader"]
pub type R = crate::R<CruSoftrstCon20Spec>;
#[doc = "Register `CRU_SOFTRST_CON20` writer"]
pub type W = crate::W<CruSoftrstCon20Spec>;
#[doc = "Field `PRESETN_GPIO2_REQ` reader - presetn_gpio2 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio2ReqR = crate::BitReader;
#[doc = "Field `PRESETN_GPIO2_REQ` writer - presetn_gpio2 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio2ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_GPIO3_REQ` reader - presetn_gpio3 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio3ReqR = crate::BitReader;
#[doc = "Field `PRESETN_GPIO3_REQ` writer - presetn_gpio3 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio3ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_GPIO4_REQ` reader - presetn_gpio4 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio4ReqR = crate::BitReader;
#[doc = "Field `PRESETN_GPIO4_REQ` writer - presetn_gpio4 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio4ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_GRF_REQ` reader - presetn_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_GRF_REQ` writer - presetn_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_ALIVE_NOC_REQ` reader - presetn_alive_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnAliveNocReqR = crate::BitReader;
#[doc = "Field `PRESETN_ALIVE_NOC_REQ` writer - presetn_alive_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnAliveNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_WDT0_REQ` reader - presetn_wdt0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnWdt0ReqR = crate::BitReader;
#[doc = "Field `PRESETN_WDT0_REQ` writer - presetn_wdt0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnWdt0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_WDT1_REQ` reader - presetn_wdt1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnWdt1ReqR = crate::BitReader;
#[doc = "Field `PRESETN_WDT1_REQ` writer - presetn_wdt1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnWdt1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_INTR_ARB_REQ` reader - presetn_intr_arb request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnIntrArbReqR = crate::BitReader;
#[doc = "Field `PRESETN_INTR_ARB_REQ` writer - presetn_intr_arb request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnIntrArbReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UPHY0_DPTX_REQ` reader - presetn_uphy0_dptx request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy0DptxReqR = crate::BitReader;
#[doc = "Field `PRESETN_UPHY0_DPTX_REQ` writer - presetn_uphy0_dptx request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy0DptxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UPHY0_APB_REQ` reader - presetn_uphy0_apb request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy0ApbReqR = crate::BitReader;
#[doc = "Field `PRESETN_UPHY0_APB_REQ` writer - presetn_uphy0_apb request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy0ApbReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UPHY0_TCPHY_REQ` reader - presetn_uphy0_tcphy request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy0TcphyReqR = crate::BitReader;
#[doc = "Field `PRESETN_UPHY0_TCPHY_REQ` writer - presetn_uphy0_tcphy request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy0TcphyReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UPHY1_TCPHY_REQ` reader - presetn_uphy1_tcphy request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy1TcphyReqR = crate::BitReader;
#[doc = "Field `PRESETN_UPHY1_TCPHY_REQ` writer - presetn_uphy1_tcphy request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy1TcphyReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UPHY0_TCPDCTRL_REQ` reader - presetn_uphy0_tcpdctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy0TcpdctrlReqR = crate::BitReader;
#[doc = "Field `PRESETN_UPHY0_TCPDCTRL_REQ` writer - presetn_uphy0_tcpdctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy0TcpdctrlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UPHY1_TCPDCTRL_REQ` reader - presetn_uphy1_tcpdctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy1TcpdctrlReqR = crate::BitReader;
#[doc = "Field `PRESETN_UPHY1_TCPDCTRL_REQ` writer - presetn_uphy1_tcpdctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUphy1TcpdctrlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - presetn_gpio2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_gpio2_req(&self) -> PresetnGpio2ReqR {
        PresetnGpio2ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - presetn_gpio3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_gpio3_req(&self) -> PresetnGpio3ReqR {
        PresetnGpio3ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - presetn_gpio4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_gpio4_req(&self) -> PresetnGpio4ReqR {
        PresetnGpio4ReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - presetn_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_grf_req(&self) -> PresetnGrfReqR {
        PresetnGrfReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - presetn_alive_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_alive_noc_req(&self) -> PresetnAliveNocReqR {
        PresetnAliveNocReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - presetn_wdt0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_wdt0_req(&self) -> PresetnWdt0ReqR {
        PresetnWdt0ReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - presetn_wdt1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_wdt1_req(&self) -> PresetnWdt1ReqR {
        PresetnWdt1ReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - presetn_intr_arb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_intr_arb_req(&self) -> PresetnIntrArbReqR {
        PresetnIntrArbReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - presetn_uphy0_dptx request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uphy0_dptx_req(&self) -> PresetnUphy0DptxReqR {
        PresetnUphy0DptxReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - presetn_uphy0_apb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uphy0_apb_req(&self) -> PresetnUphy0ApbReqR {
        PresetnUphy0ApbReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - presetn_uphy0_tcphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uphy0_tcphy_req(&self) -> PresetnUphy0TcphyReqR {
        PresetnUphy0TcphyReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - presetn_uphy1_tcphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uphy1_tcphy_req(&self) -> PresetnUphy1TcphyReqR {
        PresetnUphy1TcphyReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - presetn_uphy0_tcpdctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uphy0_tcpdctrl_req(&self) -> PresetnUphy0TcpdctrlReqR {
        PresetnUphy0TcpdctrlReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - presetn_uphy1_tcpdctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uphy1_tcpdctrl_req(&self) -> PresetnUphy1TcpdctrlReqR {
        PresetnUphy1TcpdctrlReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - presetn_gpio2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gpio2_req(&mut self) -> PresetnGpio2ReqW<CruSoftrstCon20Spec> {
        PresetnGpio2ReqW::new(self, 0)
    }
    #[doc = "Bit 1 - presetn_gpio3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gpio3_req(&mut self) -> PresetnGpio3ReqW<CruSoftrstCon20Spec> {
        PresetnGpio3ReqW::new(self, 1)
    }
    #[doc = "Bit 2 - presetn_gpio4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gpio4_req(&mut self) -> PresetnGpio4ReqW<CruSoftrstCon20Spec> {
        PresetnGpio4ReqW::new(self, 2)
    }
    #[doc = "Bit 3 - presetn_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_grf_req(&mut self) -> PresetnGrfReqW<CruSoftrstCon20Spec> {
        PresetnGrfReqW::new(self, 3)
    }
    #[doc = "Bit 4 - presetn_alive_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_alive_noc_req(&mut self) -> PresetnAliveNocReqW<CruSoftrstCon20Spec> {
        PresetnAliveNocReqW::new(self, 4)
    }
    #[doc = "Bit 5 - presetn_wdt0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_wdt0_req(&mut self) -> PresetnWdt0ReqW<CruSoftrstCon20Spec> {
        PresetnWdt0ReqW::new(self, 5)
    }
    #[doc = "Bit 6 - presetn_wdt1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_wdt1_req(&mut self) -> PresetnWdt1ReqW<CruSoftrstCon20Spec> {
        PresetnWdt1ReqW::new(self, 6)
    }
    #[doc = "Bit 7 - presetn_intr_arb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_intr_arb_req(&mut self) -> PresetnIntrArbReqW<CruSoftrstCon20Spec> {
        PresetnIntrArbReqW::new(self, 7)
    }
    #[doc = "Bit 8 - presetn_uphy0_dptx request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uphy0_dptx_req(&mut self) -> PresetnUphy0DptxReqW<CruSoftrstCon20Spec> {
        PresetnUphy0DptxReqW::new(self, 8)
    }
    #[doc = "Bit 10 - presetn_uphy0_apb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uphy0_apb_req(&mut self) -> PresetnUphy0ApbReqW<CruSoftrstCon20Spec> {
        PresetnUphy0ApbReqW::new(self, 10)
    }
    #[doc = "Bit 12 - presetn_uphy0_tcphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uphy0_tcphy_req(&mut self) -> PresetnUphy0TcphyReqW<CruSoftrstCon20Spec> {
        PresetnUphy0TcphyReqW::new(self, 12)
    }
    #[doc = "Bit 13 - presetn_uphy1_tcphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uphy1_tcphy_req(&mut self) -> PresetnUphy1TcphyReqW<CruSoftrstCon20Spec> {
        PresetnUphy1TcphyReqW::new(self, 13)
    }
    #[doc = "Bit 14 - presetn_uphy0_tcpdctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uphy0_tcpdctrl_req(&mut self) -> PresetnUphy0TcpdctrlReqW<CruSoftrstCon20Spec> {
        PresetnUphy0TcpdctrlReqW::new(self, 14)
    }
    #[doc = "Bit 15 - presetn_uphy1_tcpdctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uphy1_tcpdctrl_req(&mut self) -> PresetnUphy1TcpdctrlReqW<CruSoftrstCon20Spec> {
        PresetnUphy1TcpdctrlReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon20Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon20Spec;
impl crate::RegisterSpec for CruSoftrstCon20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con20::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon20Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con20::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON20 to value 0"]
impl crate::Resettable for CruSoftrstCon20Spec {
    const RESET_VALUE: u32 = 0;
}
